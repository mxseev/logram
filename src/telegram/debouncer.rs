use std::time::{Duration, Instant};

use crate::source::LogRecord;

struct LastMessage {
    id: i64,
    sent_at: Instant,
    title: String,
    body: Vec<String>,
}
impl LastMessage {
    fn new(id: i64, title: String, body: String) -> Self {
        let sent_at = Instant::now();
        let body = vec![body];

        LastMessage {
            id,
            sent_at,
            title,
            body,
        }
    }
}

pub enum Debounce<'a> {
    NewMessage(&'a LogRecord),
    EditMessage {
        id: i64,
        title: String,
        body: Vec<String>,
    },
}

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
    pub fn debounce<'d>(&self, record: &'d LogRecord) -> Debounce<'d> {
        if let Some(last_message) = &self.last_message {
            let title_matches = last_message.title == record.title;
            let timeout_not_elapsed = last_message.sent_at.elapsed() < self.timeout;

            if title_matches && timeout_not_elapsed {
                let mut full_body = last_message.body.clone();
                full_body.push(record.body.clone());

                return Debounce::EditMessage {
                    id: last_message.id,
                    title: record.title.clone(),
                    body: full_body,
                };
            }
        }

        Debounce::NewMessage(record)
    }
    pub fn on_message_sent(&mut self, record: LogRecord, msg_id: i64) {
        if let Some(last_message) = self.last_message.as_mut() {
            if last_message.title == record.title {
                last_message.body.push(record.body);
                last_message.sent_at = Instant::now();
            } else {
                self.set_last_message(record, msg_id);
            }
        } else {
            self.set_last_message(record, msg_id);
        }
    }
    fn set_last_message(&mut self, record: LogRecord, msg_id: i64) {
        let last_message = LastMessage::new(msg_id, record.title, record.body);
        self.last_message = Some(last_message);
    }
}
