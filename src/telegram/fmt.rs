use failure::Error;

use crate::source::LogRecord;

pub fn error(error: Error) -> String {
    format!("Error: {}", error)
}

pub fn record(record: &LogRecord) -> String {
    format!("*{}*```\n{}```", record.title, record.body)
}
