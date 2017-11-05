use std::fmt;


pub enum MessageBody {
    Error { content: String },
    FileCreated { path: String },
    FileWrited { path: String, content: String },
    FileRemoved { path: String },
}
impl fmt::Display for MessageBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            MessageBody::Error { ref content } => format!("Error: `{}`\nLogram stopped.", content),
            MessageBody::FileCreated { ref path } => format!("*{}*\nFile created.", path),
            MessageBody::FileWrited {
                ref path,
                ref content,
            } => format!("*{}*\n`{}`", path, content),
            MessageBody::FileRemoved { ref path } => format!("*{}*\nFile removed.", path),
        };
        write!(f, "{}", message)
    }
}

pub struct Message {
    pub chat: Option<i64>,
    pub body: MessageBody,
}
