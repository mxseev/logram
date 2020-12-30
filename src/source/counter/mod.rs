use futures::stream;
use std::time::Duration;
use tokio::time::delay_for;

use crate::source::{LogRecord, LogSource, LogSourceStream};

mod config;
pub use config::CounterLogSourceConfig;

pub struct CounterLogSource {
    records: i64,
    interval: Duration,
}

impl CounterLogSource {
    pub fn new(config: CounterLogSourceConfig) -> Self {
        CounterLogSource {
            records: config.initial,
            interval: Duration::from_millis(config.interval),
        }
    }
    fn next_record(&mut self) -> LogRecord {
        let title = String::from("Counter log source");
        let body = format!("It's {} record", self.records);

        self.records += 1;

        LogRecord::new(title, body)
    }
}

impl LogSource for CounterLogSource {
    fn into_stream(self) -> LogSourceStream {
        let counter = self;
        let stream = stream::unfold(counter, |mut counter| async move {
            let record = counter.next_record();
            delay_for(counter.interval).await;

            Some((Ok(record), counter))
        });

        Box::pin(stream)
    }
}
