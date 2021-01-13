use anyhow::{anyhow, Result};
use log::{self, Level, Log, Metadata, Record};
use reqwest::{
    blocking::{Client, ClientBuilder},
    Proxy, Url,
};
use serde_json::Value;
use std::time::Duration;

static USER_AGENT: &str = concat!("logram/", env!("CARGO_PKG_VERSION"));

/// Implementation of a Telegram logger for composite logger, e.g. [multi_log](https://crates.io/crates/multi_log)
pub struct TelegramLogger {
    client: Client,
    base_url: Url,
    level: Level,
    chat_id: String,
}

impl TelegramLogger {
    pub fn new(
        level: Level,
        token: String,
        chat_id: String,
        proxy: Option<String>,
    ) -> Result<Self> {
        let mut builder = ClientBuilder::new()
            .connect_timeout(Duration::from_secs(30))
            .user_agent(USER_AGENT);

        if let Some(proxy_url) = proxy {
            let proxy = Proxy::all(&proxy_url)?;

            builder = builder.proxy(proxy);
        }

        let client = builder.build()?;
        let base_url = Url::parse(&format!("https://api.telegram.org/bot{}/", token))?;

        Ok(TelegramLogger {
            client,
            base_url,
            level,
            chat_id,
        })
    }
    fn send(&self, record: &Record<'_>) -> Result<()> {
        let mut url = self.base_url.join("sendMessage")?;
        url.query_pairs_mut()
            .append_pair("parse_mode", "html")
            .append_pair("chat_id", &self.chat_id)
            .append_pair("text", &self.format_message(record));

        let response = self.client.post(url).send()?;
        if !response.status().is_success() {
            let json: Value = response.json()?;
            let description = json
                .get("description")
                .and_then(|desc| desc.as_str())
                .unwrap_or("no description")
                .to_string();

            return Err(anyhow!("API error: {}", description));
        }

        Ok(())
    }
    fn format_message(&self, record: &Record) -> String {
        format!(
            "<b>{}::{}</b><pre>{}</pre>",
            record.level(),
            record.target(),
            record.args()
        )
    }
}

impl Log for TelegramLogger {
    fn flush(&self) {}
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }
    fn log(&self, record: &Record) {
        let meta = record.metadata();
        if !self.enabled(meta) {
            return;
        }

        let blacklist_targets = &["tokio_reactor", "hyper", "mio", "want", "reqwest"];
        for target in blacklist_targets {
            if meta.target().starts_with(target) {
                return;
            }
        }

        if let Err(error) = self.send(record) {
            eprintln!("logram error: {}", error);
        }
    }
}
