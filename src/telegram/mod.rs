use anyhow::{Error, Result};
use reqwest::{ClientBuilder, Proxy};
use std::time::Duration;
use teloxide::{prelude::Request, requests::ResponseResult, types::ParseMode, Bot, BotBuilder};

mod config;
mod utils;

use crate::source::LogRecord;
pub use config::TelegramConfig;

static USER_AGENT: &str = concat!("logram/", env!("CARGO_PKG_VERSION"));

pub struct Telegram {
    bot: Bot,
    chat_id: String,
}

impl Telegram {
    pub fn new(config: TelegramConfig) -> Result<Self> {
        let mut client = ClientBuilder::new()
            .connect_timeout(Duration::from_secs(30))
            .user_agent(USER_AGENT);

        if let Some(proxy_url) = config.proxy {
            let proxy = Proxy::all(&proxy_url)?;

            client = client.proxy(proxy);
        }

        let bot = BotBuilder::new()
            .client(client.build()?)
            .token(config.token)
            .parse_mode(ParseMode::MarkdownV2)
            .build();

        Ok(Telegram {
            bot,
            chat_id: config.chat_id,
        })
    }
    pub async fn echo_id(&self) -> Result<()> {
        teloxide::repl(self.bot.clone(), |message| async move {
            let (chat_type, chat_id, title) = utils::extract_message(&message.update);

            match chat_type {
                "private" => println!("The ID of chat with @{}: {}", title, chat_id),
                "group" => println!("The chat ID of group \"{}\": {}", title, chat_id),
                "channel" => println!("The chat ID of channel \"{}\": {}", title, chat_id),
                _ => println!("I'm not entirely sure, but try this: {}", chat_id),
            };

            let text = format!("Chat ID: `{}`", chat_id);
            message.reply_to(text).send().await?;

            ResponseResult::Ok(())
        })
        .await;

        Ok(())
    }
    pub async fn send_hello(&self) -> Result<()> {
        let version = utils::escape(env!("CARGO_PKG_VERSION"));
        let hostname = utils::escape(hostname::get()?.to_string_lossy());
        let text = format!("Logram {} started at `{}`", version, hostname);

        self.send(text).await
    }
    pub async fn send_record(&self, record: LogRecord) -> Result<()> {
        let title = utils::escape(record.title);
        let body = utils::escape(record.body);
        let text = format!("*{}*```\n{}```", title, body);

        self.send(text).await
    }
    pub async fn send_error(&self, error: Error) -> Result<()> {
        let text = format!("*Error:* {}", utils::escape(error));

        self.send(text).await
    }
    async fn send(&self, text: String) -> Result<()> {
        let chat_id = self.chat_id.clone();
        self.bot.send_message(chat_id, text).send().await?;

        Ok(())
    }
}
