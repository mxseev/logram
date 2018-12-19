use systemd::journal::JournalRecord;

use crate::source::LogRecord;

#[derive(Debug)]
pub struct JournaldEvent {
    unit: String,
    message: String,
}

impl From<JournalRecord> for JournaldEvent {
    fn from(record: JournalRecord) -> Self {
        let unit = record.get("_SYSTEMD_UNIT").cloned().unwrap_or_default();
        let message = record.get("MESSAGE").cloned().unwrap_or_default();

        JournaldEvent { unit, message }
    }
}

impl Into<LogRecord> for JournaldEvent {
    fn into(self) -> LogRecord {
        LogRecord {
            title: Some(self.unit),
            body: self.message,
        }
    }
}
