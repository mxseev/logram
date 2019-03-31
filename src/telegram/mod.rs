use failure::Error;
use futures::{
    future::{self, Either},
    Future,
};

use crate::{config::TelegramConfig, source::LogRecord};

mod api;
mod fmt;
use self::api::{types::Update, TelegramApi};
pub use api::types;

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
    pub fn get_updates(token: &str) -> impl Future<Item = Vec<Update>, Error = Error> {
        let api = match TelegramApi::new(token) {
            Ok(api) => api,
            Err(error) => return Either::B(future::err(error)),
        };

        Either::A(api.get_updates(-1))
    }
    pub fn send_error(&self, error: Error) -> impl Future<Item = (), Error = Error> {
        let text = fmt::error(error);

        self.api.send_message(&self.chat_id, &text).map(|_| ())
    }
    pub fn send_record(&self, record: LogRecord) -> impl Future<Item = (), Error = Error> {
        let text = fmt::record(&record);

        self.api.send_message(&self.chat_id, &text).map(|_| ())
    }
}
