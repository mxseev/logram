use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CounterLogSourceConfig {
    pub initial: i64,
    pub interval: u64,
}

impl Default for CounterLogSourceConfig {
    fn default() -> Self {
        CounterLogSourceConfig {
            initial: 1,
            interval: 10_000,
        }
    }
}
