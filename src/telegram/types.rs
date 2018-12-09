use failure::{err_msg, Error};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<T>,
}
impl<T> Response<T> {
    pub fn into_result(self) -> Result<Option<T>, Error> {
        if self.ok {
            Ok(self.result)
        } else {
            let message = self
                .description
                .unwrap_or_else(|| String::from("no description"));

            Err(err_msg(message))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct TelegramMessage {
    pub chat: Chat,
}

#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: TelegramMessage,
}

pub type SendMessageResponse = Response<TelegramMessage>;

pub type UpdatesResponse = Response<Vec<Update>>;
