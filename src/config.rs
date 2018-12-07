use failure::Error;
use serde_derive::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub sources: LogSourcesConfig,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct LogSourcesConfig {
    pub fs: FsLogSourceConfig,
    pub journald: JournaldLogSourceConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct FsLogSourceConfig {
    pub entries: Vec<String>,
}
impl Default for FsLogSourceConfig {
    fn default() -> Self {
        FsLogSourceConfig {
            entries: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct JournaldLogSourceConfig {
    pub services: Vec<String>,
}
impl Default for JournaldLogSourceConfig {
    fn default() -> Self {
        JournaldLogSourceConfig {
            services: Vec::new(),
        }
    }
}

impl Config {
    pub fn read(filename: &str) -> Result<Self, Error> {
        let file = File::open(filename)?;
        let config: Self = serde_yaml::from_reader(file)?;

        Ok(config)
    }
}