use anyhow::Result;
use serde::Deserialize;
use std::fs::File;

use crate::{source::LogSourcesConfig, telegram::TelegramConfig};

fn default_hello() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_hello")]
    pub hello_message: bool,
    pub telegram: TelegramConfig,
    pub sources: LogSourcesConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let config: Self = serde_yaml::from_reader(file)?;

        Ok(config)
    }
}
