use std::fmt::Display;
use teloxide::types::{ChatKind, Message};

pub fn extract_message(message: &Message) -> (&'static str, i64, String) {
    let unknown_str = || String::from("<unknown>");

    if let Some(forwarded_chat) = message.forward_from_chat() {
        if forwarded_chat.is_channel() {
            if let ChatKind::Public(channel) = &forwarded_chat.kind {
                let chat_id = forwarded_chat.id;
                let title = channel.title.clone().unwrap_or_else(unknown_str);

                return ("channel", chat_id, title);
            }
        }
    }

    let id = message.chat.id;
    match &message.chat.kind {
        ChatKind::Private(chat) => {
            let username = chat.username.clone().unwrap_or_else(unknown_str);

            ("private", id, username)
        }
        ChatKind::Public(chat) => {
            let title = chat.title.clone().unwrap_or_else(unknown_str);

            ("group", id, title)
        }
        _ => ("unknown", id, unknown_str()),
    }
}

pub fn escape<S: Display>(text: S) -> String {
    text.to_string()
        .replace(".", "\\.")
        .replace("_", "\\_")
        .replace("*", "\\*")
        .replace("<", "\\<")
        .replace(">", "\\>")
        .replace("-", "\\-")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace("(", "\\(")
        .replace(")", "\\)")
}
