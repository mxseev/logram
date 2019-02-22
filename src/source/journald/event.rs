use lazy_static::lazy_static;
use regex::Regex;
use systemd::journal::JournalRecord;

use crate::source::LogRecord;

fn crop_ansi_codes(input: String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new("\x1b\\[[^@-~]*[@-~]").unwrap();
    };

    RE.replace_all(&input, "").to_string()
}

#[derive(Debug)]
pub struct JournaldEvent {
    unit: String,
    message: String,
}

impl From<JournalRecord> for JournaldEvent {
    fn from(record: JournalRecord) -> Self {
        let unit = record.get("_SYSTEMD_UNIT").cloned().unwrap_or_default();
        let message = record
            .get("MESSAGE")
            .cloned()
            .map(crop_ansi_codes)
            .unwrap_or_default();

        JournaldEvent { unit, message }
    }
}

impl Into<LogRecord> for JournaldEvent {
    fn into(self) -> LogRecord {
        LogRecord {
            title: self.unit,
            body: Some(self.message),
        }
    }
}
