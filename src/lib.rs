use failure::{err_msg, Error};
use log::{self, Level, LevelFilter, Log, Metadata, Record};
use reqwest::Client;
use serde_json::Value;

struct TelegramLogger {
    level: Level,
    token: String,
    chat_id: String,
    client: Client,
}
impl TelegramLogger {
    pub fn new(token: String, chat_id: String, level: Level) -> TelegramLogger {
        let client = Client::new();

        TelegramLogger {
            level,
            token,
            chat_id,
            client,
        }
    }
    pub fn send(&self, record: &Record) -> Result<(), Error> {
        let text = TelegramLogger::format_message(record);
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}&parse_mode=html",
            self.token, self.chat_id, text
        );

        let mut response = self.client.post(&url).send()?;
        if !response.status().is_success() {
            let json: Value = response.json()?;
            let description = json
                .get("description")
                .and_then(|desc| desc.as_str())
                .unwrap_or("no description")
                .to_string();

            return Err(err_msg(description));
        }

        Ok(())
    }
    fn format_message(record: &Record) -> String {
        let text = format!(
            "<b>{}::{}</b><pre>{}</pre>",
            record.level(),
            record.target(),
            record.args()
        );

        text
    }
}

impl Log for TelegramLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let current = log::max_level();
        log::set_max_level(LevelFilter::Off);
        if let Err(error) = self.send(record) {
            eprintln!("logram error: {}", error);
        }
        log::set_max_level(current);
    }
    fn flush(&self) {}
}

pub fn init(token: String, chat_id: String, level: Level) -> Result<(), Error> {
    let logger = TelegramLogger::new(token, chat_id, level);
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());

    Ok(())
}
