use failure::Error;
use futures::{Future, Stream};

use crate::{config::TelegramConfig, source::LogSourceEvent};

mod api;
use self::api::TelegramApi;

pub struct Telegram {
    api: TelegramApi,
    chat_id: String,
}

impl Telegram {
    pub fn new(config: TelegramConfig) -> Result<Self, Error> {
        let api = TelegramApi::new(&config.token)?;
        let chat_id = config.chat_id;

        Ok(Telegram { api, chat_id })
    }
    pub fn echo_id(token: &str) -> impl Future<Item = (), Error = Error> {
        let api = TelegramApi::new(token).unwrap();
        api.updates().for_each(|updates| {
            updates.into_iter().for_each(|update| {
                println!(
                    "[echo id]: Received message from chat with id: {}",
                    update.message.chat.id
                )
            });
            Ok(())
        })
    }
    pub fn send(&self, event: LogSourceEvent) -> impl Future<Item = (), Error = Error> {
        let text = match event {
            LogSourceEvent::Record(record) => {
                let title = record.title.unwrap_or_default();
                format!("*{}*```\n{}```", title, record.body)
            }
            LogSourceEvent::Error(error) => format!("Error: {}", error),
        };

        self.api.send_message(&self.chat_id, &text).map(|_| ())
    }
}
