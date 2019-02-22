use std::time::{Duration, Instant};

use crate::source::LogRecord;

#[derive(Debug)]
struct LastMessage {
    id: i64,
    sent_at: Instant,
    title: String,
    body: Vec<String>,
}
impl LastMessage {
    fn new(id: i64, title: String, body: Option<String>) -> Self {
        let sent_at = Instant::now();
        let body = match body {
            Some(body) => vec![body],
            None => vec![],
        };

        LastMessage {
            id,
            sent_at,
            title,
            body,
        }
    }
}

#[derive(Debug)]
pub enum Debounce<'a> {
    NewMessage(&'a LogRecord),
    EditMessage {
        id: i64,
        title: String,
        body: Vec<String>,
    },
}

#[derive(Debug)]
pub struct Debouncer {
    timeout: Duration,
    last_message: Option<LastMessage>,
}
impl Debouncer {
    pub fn new(timeout: u64) -> Self {
        let timeout = Duration::from_secs(timeout);
        let last_message = None;

        Debouncer {
            timeout,
            last_message,
        }
    }
    pub fn debounce<'a>(&self, record: &'a LogRecord) -> Debounce<'a> {
        if let Some(lm) = &self.last_message {
            if lm.title == record.title && lm.sent_at.elapsed() < self.timeout {
                let mut full_body = lm.body.clone();
                if let Some(body) = &record.body {
                    full_body.push(body.clone());
                }

                return Debounce::EditMessage {
                    id: lm.id,
                    title: record.title.clone(),
                    body: full_body,
                };
            }
        }

        Debounce::NewMessage(record)
    }
    pub fn on_message_sent(&mut self, record: LogRecord, msg_id: i64) {
        if let Some(lm) = self.last_message.as_mut() {
            if lm.title == record.title && lm.sent_at.elapsed() < self.timeout {
                if let Some(body) = record.body {
                    lm.body.push(body);
                    lm.sent_at = Instant::now();
                }
            } else {
                let lm = LastMessage::new(msg_id, record.title, record.body);
                self.last_message = Some(lm);
            }
        } else {
            let lm = LastMessage::new(msg_id, record.title, record.body);
            self.last_message = Some(lm);
        }
    }
}
