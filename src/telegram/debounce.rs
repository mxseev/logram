use std::collections::HashMap;

use crate::source::LogRecord;

use super::api::Message;

#[derive(Debug)]
pub struct DebounceInfo {
    pub message_id: i64,
    pub text: String,
}
impl DebounceInfo {
    pub fn new(message_id: i64, text: String) -> Self {
        DebounceInfo { message_id, text }
    }
}

#[derive(Debug)]
pub enum Debounce {
    SendNew { text: String },
    Update { message_id: i64, text: String },
}

pub struct Debouncer {
    debounces: HashMap<String, DebounceInfo>,
}
impl Debouncer {
    pub fn new() -> Self {
        let debounces = HashMap::new();

        Debouncer { debounces }
    }
    pub fn map_record(&self, record: &LogRecord) -> Debounce {
        let title = record.title.clone().unwrap_or_default();

        if let Some(debounce) = self.debounces.get(&title) {
            let message_id = debounce.message_id;
            let text = format!("*{}*\n```\n{}\n{}```", title, debounce.text, record.body);

            return Debounce::Update { text, message_id };
        }

        let text = format!("*{}*```\n{}```", title, record.body);
        Debounce::SendNew { text }
    }
    pub fn add_debounce(&mut self, record: LogRecord, message: &Message) {
        let title = record.title.unwrap_or_default();

        match self.debounces.get_mut(&title) {
            Some(debounce) => {
                debounce.text = format!("{}\n{}", debounce.text, record.body);
            }
            None => {
                let info = DebounceInfo::new(message.message_id, record.body);
                self.debounces.insert(title, info);
            }
        }
    }
}
