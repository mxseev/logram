use anyhow::Result;
use futures::{
    channel::{
        mpsc::{self as futures_mpsc, Receiver, Sender},
        oneshot::{self, Sender as OneshotSender},
    },
    executor,
};
use std::{iter::Iterator, thread};
use systemd::journal::{Journal, JournalFiles, JournalRecord, JournalSeek};

use crate::source::{LogRecord, LogSource, LogSourceStream};

mod config;
pub use self::config::{JournaldLogSourceConfig, MatchGroup};

pub struct JournaldLogSource {
    receiver: Receiver<Result<LogRecord>>,
}

impl JournaldLogSource {
    pub fn new(config: JournaldLogSourceConfig) -> Result<Self> {
        let (init_tx, init_rx) = oneshot::channel();
        let (record_tx, record_rx) = futures_mpsc::channel(10);

        thread::spawn(move || run_inner(config, init_tx, record_tx));

        match executor::block_on(init_rx).unwrap() {
            Err(error) => Err(error),
            Ok(_) => Ok(JournaldLogSource {
                receiver: record_rx,
            }),
        }
    }
}

impl LogSource for JournaldLogSource {
    fn into_stream(self) -> LogSourceStream {
        Box::pin(self.receiver)
    }
}

struct JournaldLogSourceInner {
    journal: Journal,
    matches: Vec<MatchGroup>,
}

impl JournaldLogSourceInner {
    fn new(config: JournaldLogSourceConfig) -> Result<Self> {
        let mut journal = Journal::open(JournalFiles::All, false, true)?;

        for (matc, is_last) in with_last(config.matches.iter()) {
            for (key, value) in &matc.filters {
                journal.match_add(&key, value.clone())?;
            }

            if !is_last {
                journal.match_or()?;
            }
        }

        journal.seek_tail()?;
        journal.seek(JournalSeek::Tail)?;

        Ok(JournaldLogSourceInner {
            journal,
            matches: config.matches,
        })
    }
    fn next_record(&mut self) -> Result<LogRecord> {
        let record = match self.journal.next_entry()? {
            Some(record) => record,
            None => {
                self.journal.wait(None)?;
                return self.next_record();
            }
        };

        let title = self.find_title(&record);
        let body = record
            .get("MESSAGE")
            .cloned()
            .unwrap_or_else(|| String::from("<unknown message>"));

        Ok(LogRecord::new(title, body))
    }
    fn find_title(&self, record: &JournalRecord) -> String {
        'outer: for matc in &self.matches {
            for (filter_key, filter_value) in &matc.filters {
                let rec_value = record.get(filter_key);
                if rec_value != Some(filter_value) {
                    continue 'outer;
                }
            }

            return matc.title.clone();
        }

        String::from("<unknown title>")
    }
}

fn run_inner(
    config: JournaldLogSourceConfig,
    init_tx: OneshotSender<Result<()>>,
    mut record_tx: Sender<Result<LogRecord>>,
) {
    let mut inner = match JournaldLogSourceInner::new(config) {
        Ok(inner) => {
            init_tx.send(Ok(())).unwrap();
            inner
        }
        Err(error) => {
            init_tx.send(Err(error)).unwrap();
            return;
        }
    };

    loop {
        let record = inner.next_record();
        if let Err(error) = record_tx.try_send(record) {
            eprintln!("Error: {}", error);
        }
    }
}

fn with_last<T, I: Iterator<Item = T>>(iter: I) -> impl Iterator<Item = (T, bool)> {
    let len = iter.size_hint().0;

    iter.enumerate().map(move |(index, t)| {
        let is_last = index + 1 == len;

        (t, is_last)
    })
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use std::collections::HashMap;
    use systemd::journal;

    use crate::source::{LogRecord, LogSource};

    use super::{JournaldLogSource, JournaldLogSourceConfig, MatchGroup};

    #[tokio::test]
    async fn main() {
        let mut filters_a = HashMap::new();
        filters_a.insert(String::from("A_FIELD1"), String::from("a_field1_value"));
        filters_a.insert(String::from("A_FIELD2"), String::from("a_field2_value"));

        let mut filters_b = HashMap::new();
        filters_b.insert(String::from("B_FIELD1"), String::from("b_field1_value"));

        let config = JournaldLogSourceConfig {
            matches: vec![
                MatchGroup {
                    title: String::from("group a"),
                    filters: filters_a,
                },
                MatchGroup {
                    title: String::from("group b"),
                    filters: filters_b,
                },
            ],
        };

        let source = JournaldLogSource::new(config).unwrap();
        let stream = source.into_stream();

        journal::send(&[
            "A_FIELD1=a_field1_value",
            "A_FIELD2=a_field2_value",
            "MESSAGE=group_a message",
            "PRIORITY=7",
        ]);

        journal::send(&[
            "B_FIELD1=another_value",
            "MESSAGE=this message should be ignored",
            "PRIORITY=7",
        ]);

        journal::send(&[
            "B_FIELD1=b_field1_value",
            "MESSAGE=group_b message",
            "PRIORITY=7",
        ]);

        let actual: Vec<LogRecord> = stream.take(2).map(Result::unwrap).collect().await;
        let expected: Vec<LogRecord> = vec![
            LogRecord::new("group a", "group_a message"),
            LogRecord::new("group b", "group_b message"),
        ];

        assert_eq!(actual, expected);
    }
}
