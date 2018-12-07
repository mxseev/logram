use systemd::journal::JournalRecord;

use crate::source::LogRecord;

#[derive(Debug)]
pub struct JournaldEvent {
    service: String,
    message: String,
}

impl From<JournalRecord> for JournaldEvent {
    fn from(record: JournalRecord) -> Self {
        let service = record.get("_SYSTEMD_UNIT").cloned().unwrap_or_default();
        let message = record.get("MESSAGE").cloned().unwrap_or_default();

        JournaldEvent { service, message }
    }
}

impl LogRecord for JournaldEvent {
    fn into_message(self) -> String {
        String::from("todo: format message")
    }
}
