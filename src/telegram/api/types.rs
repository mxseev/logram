use failure::{err_msg, Error};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<T>,
}
impl<T> Response<T> {
    pub fn into_result(self) -> Result<T, Error> {
        if self.ok {
            self.result
                .ok_or_else(|| err_msg("ok: true, but result is null"))
        } else {
            let description = self
                .description
                .unwrap_or_else(|| String::from("no description"));

            Err(err_msg(description))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub chat: Chat,
}

#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Message,
}
