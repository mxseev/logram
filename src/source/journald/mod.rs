use failure::Error;
use std::thread;
use systemd::journal::{Journal, JournalFiles, JournalSeek};

use crate::{
    config::JournaldLogSourceConfig,
    source::{LogSource, LogSourceStream},
    utils::result_channel,
};

mod record;
use self::record::map_record;

pub struct JournaldLogSource {
    journal: Journal,
}

impl JournaldLogSource {
    pub fn new(config: JournaldLogSourceConfig) -> Result<Self, Error> {
        let mut journal = Journal::open(JournalFiles::All, false, true)?;
        journal.seek(JournalSeek::Tail)?;

        for unit in config.units {
            journal.match_add("_SYSTEMD_UNIT", unit)?;
            journal.match_or()?;
        }

        Ok(JournaldLogSource { journal })
    }
}

// www.freedesktop.org/software/systemd/man/sd-journal.html#Thread%20safety
unsafe impl Send for JournaldLogSource {}

impl LogSource for JournaldLogSource {
    fn into_stream(mut self) -> Box<LogSourceStream> {
        let (tx, rx) = result_channel();
        let tx_clone = tx.clone();

        let on_record = move |record| {
            if let Some(record) = map_record(record) {
                tx.unbounded_send(Ok(record)).unwrap();
            }

            Ok(())
        };

        let thread_task = move || {
            if let Err(error) = self.journal.watch_all_elements(on_record) {
                let error = Error::from(error);
                tx_clone.unbounded_send(Err(error)).unwrap();
            }
        };

        thread::spawn(thread_task);

        Box::new(rx)
    }
}

#[cfg(test)]
mod tests {
    use futures::Stream;
    use systemd::journal;

    use crate::{
        config::JournaldLogSourceConfig,
        source::{LogRecord, LogSource},
    };

    use super::JournaldLogSource;

    fn send_log_record(keys: Vec<(&str, &str)>) {
        let keys: Vec<String> = keys
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect();

        let keys = keys.iter().map(AsRef::as_ref).collect::<Vec<_>>();

        journal::send(&keys);
    }

    fn record(title: &str, body: &str) -> LogRecord {
        LogRecord {
            title: String::from(title),
            body: String::from(body),
        }
    }

    #[test]
    fn main() {
        let config = JournaldLogSourceConfig {
            units: vec![String::from("user@1000.service")],
        };

        let source = JournaldLogSource::new(config).unwrap();
        let mut stream = source.into_stream().wait();
        let mut stream_next = || stream.next().unwrap().unwrap();

        send_log_record(vec![("MESSAGE", "logram test")]);
        assert_eq!(stream_next(), record("user@1000.service", "logram test"));

        send_log_record(vec![("MESSAGE", "logram test 2")]);
        assert_eq!(stream_next(), record("user@1000.service", "logram test 2"));
    }

}
