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

impl LogRecord for JournaldEvent {
    fn to_message(&self) -> String {
        format!("*{}*```\n{}```", self.unit, self.message)
    }
}
