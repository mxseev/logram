use std::collections::HashMap;
use std::path::PathBuf;
use std::io::{Read, Seek, SeekFrom};
use std::fs::{self, File};
use std::sync::mpsc;
use std::time::Duration;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

use telegram::{Telegram, Message, MessageBody};
use config::WatcherConfig;

mod error;
pub use self::error::WatcherError;


struct WatchingFile {
    seek: u64,
}

pub struct FileWatcher {
    telegram: Telegram,
    files: HashMap<PathBuf, WatchingFile>,
}
impl FileWatcher {
    pub fn new(config: WatcherConfig, telegram: Telegram) -> Result<FileWatcher, WatcherError> {
        let mut watcher = FileWatcher {
            telegram,
            files: HashMap::new(),
        };

        for file in config.files {
            let path = PathBuf::from(file.path);
            let meta = fs::metadata(&path)?;
            if meta.is_dir() {
                return Err(WatcherError::DirsNotSupported);
            }
            watcher.files.insert(
                path,
                WatchingFile { seek: meta.len() },
            );
        }

        Ok(watcher)
    }
    pub fn watch_files(&mut self) -> Result<(), WatcherError> {
        let channel = mpsc::channel();
        let mut watcher: RecommendedWatcher = Watcher::new(channel.0, Duration::from_secs(1))?;
        for path in self.files.keys() {
            watcher.watch(&path, RecursiveMode::NonRecursive)?;
        }

        loop {
            let event = channel.1.recv()?;
            match event {
                DebouncedEvent::Write(path) => self.on_file_writed(path)?,
                DebouncedEvent::Remove(path) => self.on_file_removed(path)?,
                _ => {}
            }
        }
    }
    fn on_file_writed(&mut self, path: PathBuf) -> Result<(), WatcherError> {
        let old_len = match self.files.get(&path) {
            Some(file) => file.seek,
            None => return Err(WatcherError::FileNotFound),
        };

        let new_len = fs::metadata(path.clone())?.len();
        let seek = if new_len > old_len {
            SeekFrom::Start(old_len)
        } else {
            SeekFrom::Start(0)
        };

        let mut file = File::open(path.clone())?;
        let mut buffer = Vec::new();
        file.seek(seek)?;
        let new_content_len = file.read_to_end(&mut buffer)?;
        let new_content = String::from_utf8_lossy(&buffer);

        let message = Message {
            chat_id: None,
            body: MessageBody::FileWrited {
                path: format!("{}", path.display()),
                content: format!("{}", new_content),
            },
        };
        self.telegram.send(message)?;

        match self.files.get_mut(&path) {
            Some(file) => file.seek += new_content_len as u64,
            None => return Err(WatcherError::FileNotFound),
        };

        Ok(())
    }
    fn on_file_removed(&self, path: PathBuf) -> Result<(), WatcherError> {
        let message = Message {
            chat_id: None,
            body: MessageBody::FileRemoved { path: format!("{}", path.display()) },
        };
        self.telegram.send(message)?;

        Ok(())
    }
}
