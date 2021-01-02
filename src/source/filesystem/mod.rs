use anyhow::Result;
use futures::channel::mpsc as futures_mpsc;
use notify::{watcher, DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use crate::source::{LogRecord, LogSource, LogSourceStream};

mod config;
mod reader;
pub use self::config::FilesystemLogSourceConfig;
use self::reader::AdditionReader;

pub struct FilesystemLogSource {
    watcher: RecommendedWatcher,
    receiver: Receiver<DebouncedEvent>,
    reader: AdditionReader,
}

impl FilesystemLogSource {
    pub fn new(config: FilesystemLogSourceConfig) -> Result<Self> {
        let (tx, receiver) = mpsc::channel();
        let delay = Duration::from_millis(config.delay);
        let mut watcher = watcher(tx, delay)?;

        for path in &config.entries {
            watcher.watch(path, RecursiveMode::Recursive)?;
        }

        let reader = AdditionReader::new(config.entries)?;

        Ok(FilesystemLogSource {
            watcher,
            receiver,
            reader,
        })
    }
    fn next_record(&mut self) -> Result<LogRecord> {
        let event = self.receiver.recv()?;

        match event {
            DebouncedEvent::Error(error, _) => Err(error.into()),
            DebouncedEvent::Create(path) => {
                self.reader.scan(path.clone())?;
                self.watcher.watch(&path, RecursiveMode::Recursive)?;

                let title = format!("{} was created", path.to_string_lossy());
                let record = LogRecord::only_title(title);

                Ok(record)
            }
            DebouncedEvent::Remove(path) => {
                self.watcher.unwatch(&path)?;

                let title = format!("{} was removed", path.to_string_lossy());
                let record = LogRecord::only_title(title);

                Ok(record)
            }
            DebouncedEvent::Rename(from, to) => {
                self.reader.scan(to.clone())?;
                self.watcher.watch(&to, RecursiveMode::Recursive)?;

                let (from, to) = (from.to_string_lossy(), to.to_string_lossy());
                let title = format!("{} was renamed to {}", from, to);
                let record = LogRecord::only_title(title);

                Ok(record)
            }
            DebouncedEvent::Write(path) => {
                let title = path.to_string_lossy().to_string();
                let body = self.reader.read_addition(path)?;
                let record = LogRecord::new(title, body);

                Ok(record)
            }
            _ => self.next_record(),
        }
    }
}

impl LogSource for FilesystemLogSource {
    fn into_stream(self) -> LogSourceStream {
        let (tx, rx) = futures_mpsc::channel(1);

        thread::spawn(move || {
            let mut tx = tx;
            let mut source = self;

            loop {
                let record = source.next_record();
                if let Err(error) = tx.try_send(record) {
                    eprintln!("Channel error: {}", error);
                }
            }
        });

        Box::pin(rx)
    }
}
