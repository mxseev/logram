use systemd::journal::JournalRecord;

use crate::{
    source::LogRecord,
    utils::{crop_ansi_codes, option_zip},
};

pub fn map_record(record: JournalRecord) -> Option<LogRecord> {
    let unit = record.get("_SYSTEMD_UNIT").cloned();
    let message = record.get("MESSAGE");

    option_zip(unit, message).map(|(unit, message)| LogRecord {
        title: unit,
        body: crop_ansi_codes(message),
    })
}
