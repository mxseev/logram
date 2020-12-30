use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelegramConfig {
    pub token: String,
    pub chat_id: String,
    pub proxy: Option<String>,
}

impl TelegramConfig {
    pub fn for_echo_id(token: String, proxy: Option<String>) -> Self {
        let chat_id = String::new();

        TelegramConfig {
            token,
            chat_id,
            proxy,
        }
    }
}
