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
    base_url: Url,
}

impl Telegram {
    pub fn new(config: TelegramConfig) -> Result<Self, Error> {
        let client = AsyncClient::new();

        let url = format!("https://api.telegram.org/bot{}/sendMessage", config.token);
        let base_url = Url::parse(&url).unwrap();

        Ok(Telegram {
            config,
            client,
            base_url,
        })
    }
    pub fn send(&self, message: &str) -> impl Future<Item = (), Error = Error> {
        let mut url = self.base_url.clone();

        url.query_pairs_mut()
            .append_pair("text", message)
            .append_pair("chat_id", &self.config.chat_id)
            .append_pair("parse_mode", "markdown");

        self.client
            .post(url)
            .send()
            .and_then(|mut response| response.json::<SendMessageResponse>())
            .map_err(Error::from)
            .and_then(|response| response.into_result())
            .map(|_| ())
    }
    pub fn echo_id(token: &str) -> Result<(), Error> {
        let url = format!("https://api.telegram.org/bot{}/getUpdates", token);
        let url = Url::parse(&url)?;
        let client = Client::new();

        let mut current_update_id = 0;

        loop {
            let mut url_clone = url.clone();
            let offset = current_update_id.to_string();
            url_clone.query_pairs_mut().append_pair("offset", &offset);

            let response: UpdatesResponse = client.get(url_clone).send()?.json()?;
            if let Some(updates) = response.into_result()? {
                for update in updates {
                    println!("[echo mode]: chat id = {}", update.message.chat.id);
                    current_update_id = update.update_id + 1;
                }
            }

            thread::sleep(Duration::from_secs(1));
        }
    }
}
