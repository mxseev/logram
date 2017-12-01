use std::fmt;


pub enum MessageBody {
    Started,
    Error { content: String },
    FileCreated { path: String },
    FileWrited { path: String, content: String },
    FileRemoved { path: String },
    Raw { content: String },
}
impl MessageBody {
    fn replaced_entities(&self) -> MessageBody {
        match *self {
            MessageBody::Started => MessageBody::Started,
            MessageBody::Error { ref content } => MessageBody::Error {
                content: replace_html_entities(content),
            },
            MessageBody::FileCreated { ref path } => MessageBody::FileCreated {
                path: replace_html_entities(path),
            },
            MessageBody::FileWrited {
                ref path,
                ref content,
            } => MessageBody::FileWrited {
                path: replace_html_entities(path),
                content: replace_html_entities(content),
            },
            MessageBody::FileRemoved { ref path } => MessageBody::FileRemoved {
                path: replace_html_entities(path),
            },
            MessageBody::Raw { ref content } => MessageBody::Raw { content: content.clone() },
        }
    }
}
impl fmt::Display for MessageBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self.replaced_entities() {
            MessageBody::Started => format!("<b>Started</b>"),
            MessageBody::Error { ref content } => {
                format!("Error: <pre>{}</pre>\nLogram stopped", content)
            }
            MessageBody::FileCreated { ref path } => format!("<b>{}</b>\nFile created", path),
            MessageBody::FileWrited {
                ref path,
                ref content,
            } => format!("<b>{}</b>\n<pre>{}</pre>", path, content),
            MessageBody::FileRemoved { ref path } => format!("<b>{}</b>\nFile removed", path),
            MessageBody::Raw { ref content } => format!("{}", content),
        };
        write!(f, "{}", message)
    }
}

pub struct Message {
    pub chat: Option<i64>,
    pub body: MessageBody,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn replace_html_entities(from: &String) -> String {
    from.replace("<", "&lt;").replace(">", "&gt;").replace("&", "&amp;")
}
