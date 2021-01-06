use anyhow::Result;
use futures::{
    channel::{
        mpsc::{self as futures_mpsc, Receiver, Sender},
        oneshot::{self, Sender as OneshotSender},
    },
    executor,
};
use std::{iter::Iterator, thread};
use systemd::journal::{Journal, JournalRecord, JournalSeek, OpenOptions};

use crate::source::{LogRecord, LogSource, LogSourceStream};

mod config;
pub use self::config::JournaldLogSourceConfig;
use self::config::MatchGroup;

pub struct JournaldLogSource {
    receiver: Receiver<Result<LogRecord>>,
}

impl JournaldLogSource {
    pub fn new(config: JournaldLogSourceConfig) -> Result<Self> {
        let (init_tx, init_rx) = oneshot::channel();
        let (record_tx, record_rx) = futures_mpsc::channel(1);

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
        let mut journal = OpenOptions::default().open()?;
        journal.seek(JournalSeek::Tail)?;

        for (matc, is_last) in with_last(config.matches.iter()) {
            for ((key, value), is_last) in with_last(matc.filters.iter()) {
                journal.match_add(&key, value.clone())?;

                if !is_last {
                    journal.match_and()?;
                }
            }

            if !is_last {
                journal.match_or()?;
            }
        }

        Ok(JournaldLogSourceInner {
            journal,
            matches: config.matches,
        })
    }
    fn next_record(&mut self) -> Result<LogRecord> {
        let record = match self.journal.await_next_entry(None)? {
            Some(record) => record,
            None => return self.next_record(),
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
        record_tx.try_send(record).unwrap();
    }
}

fn with_last<T, I: Iterator<Item = T>>(iter: I) -> impl Iterator<Item = (T, bool)> {
    let len = iter.size_hint().0;

    iter.enumerate().map(move |(index, t)| {
        let is_last = index + 1 == len;

        (t, is_last)
    })
}
