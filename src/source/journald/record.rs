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

pub fn map_record(record: JournalRecord) -> LogRecord {
    let title = record.get("_SYSTEMD_UNIT").cloned().unwrap_or_default();
    let body = record
        .get("MESSAGE")
        .cloned()
        .map(crop_ansi_codes)
        .unwrap_or_default();

    LogRecord { title, body }
}
