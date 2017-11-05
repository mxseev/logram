use std::collections::HashMap;
use std::path::PathBuf;
use std::io::{Read, Seek, SeekFrom};
use std::fs::{self, File};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use regex::Regex;

use telegram::{Telegram, Message, MessageBody};
use config::WatcherConfig;

mod error;
pub use self::error::WatcherError;


struct WatchingFile {
    seek: u64,
    chat: Option<i64>,
    regex: Option<Regex>,
}

pub struct FileWatcher {
    telegram: Telegram,
    files: HashMap<PathBuf, WatchingFile>,
    watcher: RecommendedWatcher,
    watcher_receiver: Receiver<DebouncedEvent>,
}
impl FileWatcher {
    pub fn new(config: WatcherConfig, telegram: Telegram) -> Result<FileWatcher, WatcherError> {
        let chan = mpsc::channel();
        let mut watcher = FileWatcher {
            telegram,
            files: HashMap::new(),
            watcher: Watcher::new(chan.0, Duration::from_secs(1))?,
            watcher_receiver: chan.1,
        };

        for file in config.files {
            let path = PathBuf::from(file.path);
            let meta = fs::metadata(&path)?;
            if meta.is_dir() {
                return Err(WatcherError::DirsNotSupported);
            }

            let regex = match file.regex {
                Some(regex) => Some(Regex::new(&regex)?),
                None => None,
            };

            watcher.files.insert(
                path,
                WatchingFile {
                    seek: meta.len(),
                    chat: file.chat,
                    regex: regex,
                },
            );
        }

        Ok(watcher)
    }
    pub fn watch_files(&mut self) -> Result<(), WatcherError> {
        for path in self.files.keys() {
            self.watcher.watch(&path, RecursiveMode::NonRecursive)?;
        }

        loop {
            let event = self.watcher_receiver.recv()?;
            match event {
                DebouncedEvent::Create(path) => self.on_file_created(path)?,
                DebouncedEvent::Write(path) => self.on_file_writed(path)?,
                DebouncedEvent::Remove(path) => self.on_file_removed(path)?,
                _ => {}
            }
        }
    }
    fn on_file_writed(&mut self, path: PathBuf) -> Result<(), WatcherError> {
        if let Some(watching_file) = self.files.get_mut(&path) {
            let old_len = watching_file.seek;
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
            if new_content == String::new() {
                return Ok(());
            }

            if let Some(ref regex) = watching_file.regex {
                if !regex.is_match(&new_content) {
                    return Ok(());
                }
            }

            let message = Message {
                chat: watching_file.chat,
                body: MessageBody::FileWrited {
                    path: format!("{}", path.display()),
                    content: format!("{}", new_content),
                },
            };
            self.telegram.send(message)?;
            watching_file.seek += new_content_len as u64
        }

        Ok(())
    }
    fn on_file_removed(&mut self, path: PathBuf) -> Result<(), WatcherError> {
        if let Some(watching_file) = self.files.get_mut(&path) {
            let message = Message {
                chat: watching_file.chat,
                body: MessageBody::FileRemoved { path: format!("{}", path.display()) },
            };
            self.telegram.send(message)?;

            let path_clone = path.clone();
            let parent_dir = match path_clone.parent() {
                Some(parent) => parent,
                None => return Err(WatcherError::ParentDirNotFound),
            };
            self.watcher.watch(&parent_dir, RecursiveMode::NonRecursive)?;
        };

        Ok(())
    }
    fn on_file_created(&mut self, path: PathBuf) -> Result<(), WatcherError> {
        if let Some(watching_file) = self.files.get(&path) {
            let message = Message {
                chat: watching_file.chat,
                body: MessageBody::FileCreated { path: format!("{}", path.display()) },
            };
            self.telegram.send(message)?;

            let path_clone = path.clone();
            let parent_dir = match path_clone.parent() {
                Some(parent) => parent,
                None => return Err(WatcherError::ParentDirNotFound),
            };
            self.watcher.unwatch(&parent_dir)?;
            self.watcher.watch(&path, RecursiveMode::NonRecursive)?;
        }
        self.on_file_writed(path)?;

        Ok(())
    }
}
