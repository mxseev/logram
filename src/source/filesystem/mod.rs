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
            DebouncedEvent::Remove(path) | DebouncedEvent::NoticeRemove(path) => {
                self.watcher.unwatch(&path)?;

                let title = format!("{} was removed", path.to_string_lossy());
                let record = LogRecord::only_title(title);

                Ok(record)
            }
            DebouncedEvent::Rename(from, to) => {
                self.reader.scan(to.clone())?;
                self.watcher.unwatch(&from)?;
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
        let (tx, rx) = futures_mpsc::channel(10);

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

#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use std::{
        env,
        fs::{self, File},
        io::Write,
        thread,
        time::Duration,
    };

    use crate::source::{LogRecord, LogSource};

    use super::{FilesystemLogSource, FilesystemLogSourceConfig};

    #[tokio::test]
    async fn main() {
        let base_path = env::temp_dir().join("logram_test");
        if base_path.exists() {
            fs::remove_dir_all(&base_path).unwrap();
        }
        fs::create_dir_all(&base_path).unwrap();

        let dir_path = base_path.join("dir");
        let file_a_path = base_path.join("file_a");
        let file_b_path = base_path.join("file_b");
        let file_c_path = dir_path.join("file_c");

        let file_a_path_string = file_a_path.to_string_lossy().to_string();
        let file_b_path_string = file_b_path.to_string_lossy().to_string();
        let file_c_path_string = file_c_path.to_string_lossy().to_string();

        fs::create_dir_all(&dir_path).unwrap();
        let mut file_a = File::create(&file_a_path).unwrap();
        let mut file_b = File::create(&file_b_path).unwrap();

        let config = FilesystemLogSourceConfig {
            delay: 100,
            entries: vec![dir_path.clone(), file_a_path.clone(), file_b_path.clone()],
        };

        let source = FilesystemLogSource::new(config).unwrap();
        let stream = source.into_stream();

        let _ = file_a.write(b"file_a addition").unwrap();
        let _ = file_b.write(b"file_b addition").unwrap();
        let mut file_c = File::create(&file_c_path).unwrap();
        thread::sleep(Duration::from_secs(1));
        let _ = file_c.write(b"file_c addition").unwrap();

        let actual: Vec<LogRecord> = stream.take(4).map(Result::unwrap).collect().await;
        let expected: Vec<LogRecord> = vec![
            LogRecord::new(&file_a_path_string, "file_a addition"),
            LogRecord::new(&file_b_path_string, "file_b addition"),
            LogRecord::new(&format!("{} was created", file_c_path_string), ""),
            LogRecord::new(&file_c_path_string, "file_c addition"),
        ];

        assert_eq!(actual, expected);
    }
}
