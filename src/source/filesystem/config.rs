use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct FilesystemLogSourceConfig {
    pub delay: u64,
    pub entries: Vec<PathBuf>,
}

impl Default for FilesystemLogSourceConfig {
    fn default() -> Self {
        FilesystemLogSourceConfig {
            delay: 1_000,
            entries: Vec::new(),
        }
    }
}
