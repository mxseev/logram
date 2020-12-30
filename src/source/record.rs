#[derive(Debug)]
pub struct LogRecord {
    title: Option<String>,
    body: String,
}

impl LogRecord {
    pub fn new(title: String, body: String) -> Self {
        LogRecord {
            title: Some(title),
            body,
        }
    }
    pub fn format(&self) -> String {
        match &self.title {
            Some(title) => format!("*{}*```\n{}```", title, self.body),
            None => format!("```{}```", self.body),
        }
    }
}
