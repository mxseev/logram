use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct MatchGroup {
    pub title: String,
    pub filters: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct JournaldLogSourceConfig {
    pub matches: Vec<MatchGroup>,
}

impl Default for JournaldLogSourceConfig {
    fn default() -> Self {
        JournaldLogSourceConfig {
            matches: Vec::new(),
        }
    }
}
