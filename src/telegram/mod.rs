use failure::Error;
use futures::{
    future::{self, Either},
    stream, Future, Stream,
};
use std::sync::{Arc, Mutex};

use crate::{config::TelegramConfig, source::LogRecord};

mod api;
mod debouncer;
mod fmt;
use self::{
    api::TelegramApi,
    debouncer::{Debounce, Debouncer},
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

        let debouncer = Debouncer::new(config.debounce_timeout);
        let debouncer = Arc::new(Mutex::new(debouncer));

        Ok(Telegram {
            api,
            chat_id,
            debouncer,
        })
    }
    pub fn echo_id(token: &str) -> impl Future<Item = (), Error = Error> {
        let api = match TelegramApi::new(token) {
            Ok(api) => api,
            Err(error) => return Either::B(future::err(error)),
        };

        let updates_stream = api
            .updates()
            .map(stream::iter_ok)
            .flatten()
            .for_each(|update| {
                println!(
                    "[echo id]: Received message from chat with id: {}",
                    update.message.chat.id
                );
                Ok(())
            });

        Either::A(updates_stream)
    }
    pub fn send_error(&self, error: Error) -> impl Future<Item = (), Error = Error> {
        let text = fmt::error(error);

        self.api.send_message(&self.chat_id, &text).map(|_| ())
    }
    pub fn send_record(&self, record: LogRecord) -> impl Future<Item = (), Error = Error> {
        let debouncer = self.debouncer.lock().unwrap();

        let fut = match debouncer.debounce(&record) {
            Debounce::NewMessage(record) => {
                let text = fmt::record(record);
                let fut = self.api.send_message(&self.chat_id, &text);

                Either::A(fut)
            }
            Debounce::EditMessage { id, title, body } => {
                let text = fmt::debounce(title, body);
                let fut = self.api.edit_message(&self.chat_id, id, &text);

                Either::B(fut)
            }
        };

        let debouncer = self.debouncer.clone();
        fut.and_then(move |msg| {
            let mut debouncer = debouncer.lock().unwrap();
            debouncer.on_message_sent(record, msg.message_id);

            Ok(())
        })
    }
}
