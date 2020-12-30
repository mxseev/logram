use anyhow::{Error, Result};
use reqwest::{ClientBuilder, Proxy};
use std::time::Duration;
use teloxide::{
    prelude::Request,
    requests::ResponseResult,
    types::ParseMode,
    types::{ChatKind, Message},
    Bot, BotBuilder,
};

mod config;

use crate::source::{LogRecord, LogSourcesConfig};
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
            let (chat_type, chat_id, title) = extract(&message.update);

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
    pub async fn send_hello(&self, config: &LogSourcesConfig) -> Result<()> {
        let text = format_hello(config)?;

        self.send(text).await
    }
    pub async fn send_record(&self, record: LogRecord) -> Result<()> {
        let text = record.format();

        self.send(text).await
    }
    pub async fn send_error(&self, error: Error) -> Result<()> {
        let text = format!("*Error:* {:?}", error);

        self.send(text).await
    }
    async fn send(&self, text: String) -> Result<()> {
        let chat_id = self.chat_id.clone();
        self.bot.send_message(chat_id, text).send().await?;

        Ok(())
    }
}

fn extract(message: &Message) -> (&'static str, i64, String) {
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

fn format_hello(config: &LogSourcesConfig) -> Result<String> {
    let version = env!("CARGO_PKG_VERSION").replace(".", "\\.");
    let hostname = hostname::get()?.to_string_lossy().into_owned();
    let header = format!("Logram v{} started at {}", version, hostname);

    let mut log_sources = String::from("Enabled log sources: \n");

    if config.counter.enabled {
        log_sources.push_str(&format!(
            "– Counter: `interval \\= {}, initial \\= {}`",
            config.counter.inner.interval, config.counter.inner.initial
        ));
    }

    let hello = format!("{}\n\n{}", header, log_sources);

    Ok(hello)
}
