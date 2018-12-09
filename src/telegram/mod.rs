use failure::Error;
use futures::Future;
use reqwest::{r#async::Client as AsyncClient, Client};
use std::{thread, time::Duration};
use url::Url;

use crate::config::TelegramConfig;

mod types;
use self::types::{SendMessageResponse, UpdatesResponse};

pub struct Telegram {
    config: TelegramConfig,
    client: AsyncClient,
}

impl Telegram {
    pub fn new(config: TelegramConfig) -> Result<Self, Error> {
        let client = AsyncClient::new();

        Ok(Telegram { config, client })
    }
    pub fn send(&self, message: &str) -> impl Future<Item = (), Error = Error> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.config.token
        );
        let mut url = Url::parse(&url).unwrap();

        let chat_id = self.config.chat_id.to_string();
        url.query_pairs_mut().append_pair("text", message);
        url.query_pairs_mut().append_pair("chat_id", &chat_id);
        url.query_pairs_mut().append_pair("parse_mode", "markdown");

        self.client
            .post(url)
            .send()
            .and_then(|mut response| response.json::<SendMessageResponse>())
            .map_err(Error::from)
            .and_then(|response| response.into_result())
            .map(|_| ())
    }
    pub fn echo_id(token: &str) -> Result<(), Error> {
        let client = Client::new();
        let url = format!("https://api.telegram.org/bot{}/getUpdates", token);
        let url = Url::parse(&url)?;

        let mut current_update_id = 0;

        loop {
            let mut url_clone = url.clone();
            url_clone
                .query_pairs_mut()
                .append_pair("offset", &current_update_id.to_string());

            let response: UpdatesResponse = client.get(url_clone).send()?.json()?;
            if let Some(updates) = response.into_result()? {
                for update in updates {
                    println!("[echo id mode]: chat id {}", update.message.chat.id);

                    current_update_id = update.update_id + 1;
                }
            }

            thread::sleep(Duration::from_secs(1));
        }
    }
}
