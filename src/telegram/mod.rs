use failure::Error;
use futures::{future::Either, Future};
use std::sync::{Arc, Mutex};

use crate::{config::TelegramConfig, source::LogSourceEvent};

mod api;
mod debounce;
use self::{
    api::TelegramApi,
    debounce::{Debounce, Debouncer},
};

pub struct Telegram {
    api: TelegramApi,
    chat_id: String,
    debouncer: Arc<Mutex<Debouncer>>,
}

impl Telegram {
    pub fn new(config: TelegramConfig) -> Result<Self, Error> {
        let api = TelegramApi::new(&config.token)?;
        let chat_id = config.chat_id;
        let debouncer = Arc::new(Mutex::new(Debouncer::new()));

        Ok(Telegram {
            api,
            chat_id,
            debouncer,
        })
    }
    pub fn send(&self, event: LogSourceEvent) -> impl Future<Item = (), Error = Error> {
        match event {
            LogSourceEvent::Record(record) => {
                let future = match self.debouncer.lock().unwrap().map_record(&record) {
                    Debounce::SendNew { text } => {
                        Either::A(self.api.send_message(&self.chat_id, &text))
                    }
                    Debounce::Update { message_id, text } => {
                        Either::B(self.api.edit_message(&self.chat_id, message_id, &text))
                    }
                };

                let debouncer = self.debouncer.clone();
                let future = future.and_then(move |message| {
                    debouncer.lock().unwrap().add_debounce(record, &message);
                    Ok(())
                });

                Either::A(future)
            }
            LogSourceEvent::Error(error) => {
                let text = format!("Error: {}", error);
                let future = self.api.send_message(&self.chat_id, &text).map(|_| ());

                Either::B(future)
            }
        }
    }
}
