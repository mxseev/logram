#[derive(Debug, PartialEq)]
pub struct LogRecord {
    pub title: String,
    pub body: String,
}

impl LogRecord {
    pub fn new(title: String, body: String) -> Self {
        LogRecord { title, body }
    }
    pub fn only_title(title: String) -> Self {
        LogRecord {
            title,
            body: String::new(),
        }
    }
}
