use failure::Error;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};
use walkdir::WalkDir;

use crate::{
    config::FsLogSourceConfig,
    source::{LogSource, LogSourceStream},
    utils::result_channel,
};

mod event;
use self::event::FsEvent;

pub struct FsLogSource {
    sizes: HashMap<PathBuf, u64>,
    _watcher: RecommendedWatcher,
    receiver: Receiver<DebouncedEvent>,
}

impl FsLogSource {
    pub fn new(config: FsLogSourceConfig) -> Result<FsLogSource, Error> {
        let (tx, receiver) = mpsc::channel();
        let debounce_interval = Duration::from_secs(1);
        let mut watcher: RecommendedWatcher = Watcher::new(tx, debounce_interval)?;

        let mut sizes = HashMap::new();

        for path in config.entries {
            let path = PathBuf::from(path);
            let mode = RecursiveMode::Recursive;

            watcher.watch(&path, mode)?;
            for children in WalkDir::new(path) {
                let path = children?.into_path();
                let meta = path.metadata()?;
                if meta.is_file() {
                    sizes.insert(path, meta.len());
                }
            }
        }

        Ok(FsLogSource {
            sizes,
            _watcher: watcher,
            receiver,
        })
    }
    fn map_write_event(&mut self, path: PathBuf) -> Result<FsEvent, Error> {
        let mut old_size = self.sizes.get(&path).cloned().unwrap_or(0);
        let meta = path.metadata()?;
        if meta.len() < old_size {
            old_size = 0;
        }

        let mut buffer = Vec::new();
        let mut file = File::open(&path)?;
        file.seek(SeekFrom::Start(old_size))?;

        let new_content_size = file.read_to_end(&mut buffer)?;
        let new_content = String::from_utf8_lossy(&buffer).to_string();

        if let Some(size) = self.sizes.get_mut(&path) {
            *size = old_size + new_content_size as u64;
        }

        Ok(FsEvent::Writed { path, new_content })
    }
    fn map_event(&mut self, event: DebouncedEvent) -> Result<Option<FsEvent>, Error> {
        let event = match event {
            DebouncedEvent::Create(path) => Some(FsEvent::Created { path }),
            DebouncedEvent::Write(path) => Some(self.map_write_event(path)?),
            DebouncedEvent::Remove(path) => Some(FsEvent::Removed { path }),
            DebouncedEvent::Rename(from, to) => Some(FsEvent::Renamed { from, to }),
            _ => None,
        };

        Ok(event)
    }
    fn next_event(&mut self) -> Result<FsEvent, Error> {
        let event = self.receiver.recv()?;
        if let DebouncedEvent::Error(error, _) = event {
            return Err(Error::from(error));
        }

        if let Some(mapped_event) = self.map_event(event)? {
            return Ok(mapped_event);
        }

        self.next_event()
    }
}

impl LogSource for FsLogSource {
    fn into_stream(mut self) -> Box<LogSourceStream> {
        let (tx, rx) = result_channel();

        thread::spawn(move || loop {
            let result = self.next_event().map(FsEvent::into_record);
            tx.unbounded_send(result).unwrap();
        });

        Box::new(rx)
    }
}
