use failure::Error;

use crate::source::LogRecord;

pub fn error(error: Error) -> String {
    format!("Error: {}", error)
}

pub fn record(record: &LogRecord) -> String {
    match &record.body {
        Some(body) => format!("*{}*```\n{}```", record.title, body),
        None => format!("*{}*", record.title),
    }
}

pub fn debounce(title: String, body: Vec<String>) -> String {
    format!("*{}*```\n{}```", title, body.join("\n"))
}
