extern crate slog;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate url;
extern crate strfmt;

use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use slog::{Drain, Record, OwnedKVList, Key, Serializer, KV};

mod telegram;
use telegram::{Telegram, TelegramError, Message, MessageBody, replace_html_entities};
pub use telegram::Config;


static DEFAULT_FMT_MESSAGE: &str = "{global_kv}\n\n{record}{record_kv}";
static DEFAULT_FMT_RECORD: &str = "<b>{level} {file}:{line}</b>\n<pre>{msg}</pre>";
static DEFAULT_FMT_KV: &str = "<b>{key}:</b> {val}";

pub struct Formats<'a> {
    pub message: Option<&'a str>,
    pub record: Option<&'a str>,
    pub kv: Option<&'a str>,
}

#[derive(Debug)]
pub enum TelegramDrainError {
    Telegram(TelegramError),
    Format(strfmt::FmtError),
    Slog(slog::Error),
}
impl fmt::Display for TelegramDrainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cause = match self.cause() {
            Some(e) => format!(" ({})", e),
            None => String::new(),
        };
        write!(f, "{}{}", self.description(), &cause)
    }
}
impl Error for TelegramDrainError {
    fn description(&self) -> &str {
        match *self {
            TelegramDrainError::Telegram(_) => "Telegram error",
            TelegramDrainError::Format(_) => "Format error",
            TelegramDrainError::Slog(_) => "Slog error",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            TelegramDrainError::Telegram(ref e) => Some(e),
            TelegramDrainError::Format(ref e) => Some(e),
            TelegramDrainError::Slog(ref e) => Some(e),
        }
    }
}
impl From<TelegramError> for TelegramDrainError {
    fn from(e: TelegramError) -> TelegramDrainError {
        TelegramDrainError::Telegram(e)
    }
}
impl From<strfmt::FmtError> for TelegramDrainError {
    fn from(e: strfmt::FmtError) -> TelegramDrainError {
        TelegramDrainError::Format(e)
    }
}
impl From<slog::Error> for TelegramDrainError {
    fn from(e: slog::Error) -> TelegramDrainError {
        TelegramDrainError::Slog(e)
    }
}

macro_rules! params {
    ($( $key:expr => $val:expr ),*) => {{
        let mut params = HashMap::new();
        $(
            let key = replace_html_entities(&format!("{}", $key));
            let val = replace_html_entities(&format!("{}", $val));
            params.insert(key, val);
        )*
        params
    }}
}

pub struct KvSerializer<'a> {
    buffer: String,
    format: &'a str,
}
impl<'a> KvSerializer<'a> {
    fn new(format: &'a str) -> KvSerializer {
        KvSerializer {
            buffer: String::new(),
            format,
        }
    }
    fn buffer(&self) -> String {
        self.buffer.clone()
    }
}
impl<'a> Serializer for KvSerializer<'a> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn emit_arguments(&mut self, key: Key, val: &fmt::Arguments) -> Result<(), slog::Error> {
        let params = params!{
            "key" => key,
            "val" => val
        };
        let formated = match strfmt::strfmt(self.format, &params) {
            Ok(line) => line,
            Err(_) => return Err(slog::Error::Other),
        };
        self.buffer = format!("{}\n{}", self.buffer, formated);

        Ok(())
    }
}

type DrainResult<T> = Result<T, TelegramDrainError>;

pub struct TelegramDrain<'a> {
    telegram: Telegram,
    message_format: &'a str,
    record_format: &'a str,
    kv_format: &'a str,
}
impl<'a> TelegramDrain<'a> {
    pub fn new(config: Config, formats: Formats<'a>) -> DrainResult<TelegramDrain<'a>> {
        Ok(TelegramDrain {
            telegram: Telegram::new(config)?,
            message_format: formats.message.unwrap_or(DEFAULT_FMT_MESSAGE),
            record_format: formats.record.unwrap_or(DEFAULT_FMT_RECORD),
            kv_format: formats.kv.unwrap_or(DEFAULT_FMT_KV),
        })
    }
    fn format_global_kv(&self, record: &Record, values: &OwnedKVList) -> DrainResult<String> {
        let mut kv_serializer = KvSerializer::new(self.kv_format);
        values.serialize(record, &mut kv_serializer)?;

        Ok(kv_serializer.buffer())
    }
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn format_record(&self, record: &Record) -> DrainResult<String> {
        let params = params!{
            "msg" => record.msg(),
            "level" => record.level(),
            "line" => record.line(),
            "column" => record.column(),
            "file" => record.file(),
            "tag" => record.tag(),
            "module" => record.module(),
            "function" => record.function()
        };
        let formated = strfmt::strfmt(self.record_format, &params)?;

        Ok(formated)
    }
    fn format_record_kv(&self, record: &Record) -> DrainResult<String> {
        let mut kv_serializer = KvSerializer::new(self.kv_format);
        record.kv().serialize(record, &mut kv_serializer)?;

        Ok(kv_serializer.buffer())
    }
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn format_message(&self, record: &Record, values: &OwnedKVList) -> DrainResult<String> {
        let mut params = HashMap::new();
        params.insert("global_kv".to_string(), self.format_global_kv(record, values)?);
        params.insert("record".to_string(), self.format_record(record)?);
        params.insert("record_kv".to_string(), self.format_record_kv(record)?);

        let formated = strfmt::strfmt(self.message_format, &params)?;

        Ok(formated)
    }
}
impl<'a> Drain for TelegramDrain<'a> {
    type Ok = ();
    type Err = TelegramDrainError;

    fn log(&self, record: &Record, values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        let content = self.format_message(record, values)?;
        let message = Message {
            chat: None,
            body: MessageBody::Raw { content },
        };
        self.telegram.send(message)?;

        Ok(())
    }
}
