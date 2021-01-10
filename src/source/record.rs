#[derive(Debug, PartialEq)]
pub struct LogRecord {
    pub title: String,
    pub body: String,
}

impl LogRecord {
    pub fn new<Ts: Into<String>, Bs: Into<String>>(title: Ts, body: Bs) -> Self {
        LogRecord {
            title: title.into(),
            body: body.into(),
        }
    }
    pub fn only_title(title: String) -> Self {
        LogRecord {
            title,
            body: String::new(),
        }
    }
}
