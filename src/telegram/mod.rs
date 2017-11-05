use reqwest::{self, Client};
use url::Url;
use serde_json;

use config::TelegramConfig;

mod error;
mod message;
pub use self::error::{TelegramError, ApiError};
pub use self::message::{Message, MessageBody};


#[derive(Debug, Deserialize)]
pub struct Response<T> {
    ok: bool,
    description: Option<String>,
    result: Option<T>,
}
#[derive(Debug, Deserialize)]
pub struct Chat {
    id: i64,
}
#[derive(Debug, Deserialize)]
pub struct TelegramMessage {
    chat: Chat,
    forward_from_chat: Option<Chat>,
}
#[derive(Debug, Deserialize)]
pub struct Update {
    update_id: i64,
    message: TelegramMessage,
}
pub type SendMessageResponse = Response<TelegramMessage>;
pub type UpdatesResponse = Response<Vec<Update>>;


#[derive(Clone)]
pub struct Telegram {
    client: Client,
    base_url: Url,
    default_chat: i64,
}
impl Telegram {
    pub fn new(config: TelegramConfig) -> Result<Telegram, TelegramError> {
        let url = format!("https://api.telegram.org/bot{}/", config.token);

        Ok(Telegram {
            client: Client::new(),
            base_url: Url::parse(&url)?,
            default_chat: config.chat,
        })
    }
    pub fn send(&self, message: Message) -> Result<(), TelegramError> {
        let text = message.body.to_string();
        let chat_id = match message.chat {
            Some(id) => id.to_string(),
            None => self.default_chat.to_string(),
        };

        let mut url = self.base_url.join("sendMessage")?;
        url.query_pairs_mut().append_pair("text", &text);
        url.query_pairs_mut().append_pair("chat_id", &chat_id);
        url.query_pairs_mut().append_pair("parse_mode", "markdown");

        let response = self.client.post(url).send()?;
        let response: SendMessageResponse = serde_json::from_reader(response)?;
        if !response.ok {
            return Err(TelegramError::Api(ApiError::from(response)));
        }

        Ok(())
    }
    pub fn echo_id(token: String) -> Result<(), TelegramError> {
        let base_url = Url::parse(&format!("https://api.telegram.org/bot{}/", token))?;
        let mut last_update: Option<i64> = None;

        loop {
            let mut url = base_url.join("getUpdates")?;
            url.query_pairs_mut().append_pair("timeout", "10");
            url.query_pairs_mut().append_pair(
                "allowed_updates",
                "[\"message\"]",
            );

            if let Some(last_update_id) = last_update {
                let offset = (last_update_id + 1).to_string();
                url.query_pairs_mut().append_pair("offset", &offset);
            }

            let response = reqwest::get(url.clone())?;
            let response: UpdatesResponse = serde_json::from_reader(response)?;
            if !response.ok {
                return Err(TelegramError::Api(ApiError::from(response)));
            }

            if let Some(updates) = response.result {
                for update in updates {
                    last_update = Some(update.update_id);
                    let chat_id = match update.message.forward_from_chat {
                        Some(ref forwarded_chat) => forwarded_chat.id,
                        None => update.message.chat.id,
                    };
                    println!("[echo mode] chat id: {}", chat_id);
                }
            }
        }
    }
}
