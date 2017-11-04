use std::fmt;


pub enum MessageBody {
    ChatId(i64),
}
impl fmt::Display for MessageBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            MessageBody::ChatId(id) => format!("Chat ID: `{}`", id),
        };
        write!(f, "{}", message)
    }
}

pub struct Message {
    pub chat_id: Option<i64>,
    pub body: MessageBody,
}
impl Message {
    pub fn new(chat_id: Option<i64>, body: MessageBody) -> Message {
        Message { chat_id, body }
    }
}
