use reqwest::{Client, ClientBuilder};
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
    message: Option<TelegramMessage>,
}
pub type SendMessageResponse = Response<TelegramMessage>;
pub type UpdatesResponse = Response<Vec<Update>>;

pub struct Telegram {
    client: Client,
    base_url: Url,
    default_chat: String,
    last_update: Option<i64>,
}
impl Telegram {
    pub fn new(config: TelegramConfig) -> Result<Telegram, TelegramError> {
        let url = format!("https://api.telegram.org/bot{}/", config.token);

        Ok(Telegram {
            client: ClientBuilder::new().timeout(None).build()?,
            base_url: Url::parse(&url)?,
            default_chat: config.chat,
            last_update: None,
        })
    }
    pub fn send(&mut self, message: Message) -> Result<(), TelegramError> {
        let text = message.body.to_string();
        let chat_id = match message.chat_id {
            Some(id) => id.to_string(),
            None => self.default_chat.clone(),
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
    pub fn listen_updates(&mut self) -> Result<(), TelegramError> {
        loop {
            let mut url = self.base_url.join("getUpdates")?;
            url.query_pairs_mut().append_pair("timeout", "60");
            url.query_pairs_mut().append_pair(
                "allowed_updates",
                "[\"message\"]",
            );

            if let Some(last_update_id) = self.last_update {
                let offset = (last_update_id + 1).to_string();
                url.query_pairs_mut().append_pair("offset", &offset);
            }

            let response = self.client.get(url.clone()).send()?;
            let response: UpdatesResponse = serde_json::from_reader(response)?;
            if !response.ok {
                return Err(TelegramError::Api(ApiError::from(response)));
            }

            if let Some(updates) = response.result {
                for update in updates {
                    self.last_update = Some(update.update_id);
                    self.on_update(&update)?;
                }
            }
        }
    }
    fn on_update(&mut self, update: &Update) -> Result<(), TelegramError> {
        if let Some(ref message) = update.message {
            let chat_id = match message.forward_from_chat {
                Some(ref forwarded_chat) => forwarded_chat.id,
                None => message.chat.id,
            };
            let reply = Message::new(Some(message.chat.id), MessageBody::ChatId(chat_id));
            self.send(reply)?;
        }

        Ok(())
    }
}
