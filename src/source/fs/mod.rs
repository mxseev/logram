use failure::Error;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, SeekFrom},
    mem,
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

        mem::forget(watcher);

        Ok(FsLogSource { sizes, receiver })
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

#[cfg(test)]
mod tests {
    use futures::Stream;
    use std::{
        env,
        fs::{self, File},
    };

    use crate::source::{LogRecord, LogSource};

    use super::{FsLogSource, FsLogSourceConfig};

    fn record(title: &str, body: &str) -> LogRecord {
        LogRecord {
            title: String::from(title),
            body: String::from(body),
        }
    }

    #[test]
    fn main() {
        let base_path = env::temp_dir().join("logram_test");
        if base_path.exists() {
            fs::remove_dir_all(&base_path).unwrap();
        }
        fs::create_dir(&base_path).unwrap();

        let base_path_string = base_path.display().to_string();
        let config = FsLogSourceConfig {
            entries: vec![base_path_string],
        };

        let source = FsLogSource::new(config).unwrap();
        let mut stream = source.into_stream().wait();

        let file_path = base_path.join("file");
        let new_file_path = base_path.join("file_renamed");

        File::create(&file_path).unwrap();
        assert_eq!(
            stream.next().unwrap().unwrap(),
            record("Filesystem", "/tmp/logram_test/file created")
        );

        fs::write(&file_path, b"content").unwrap();
        assert_eq!(
            stream.next().unwrap().unwrap(),
            record("/tmp/logram_test/file", "content")
        );

        fs::rename(&file_path, &new_file_path).unwrap();
        assert_eq!(
            stream.next().unwrap().unwrap(),
            record(
                "Filesystem",
                "/tmp/logram_test/file renamed to /tmp/logram_test/file_renamed",
            )
        );

        fs::remove_file(&new_file_path).unwrap();
        assert_eq!(
            stream.next().unwrap().unwrap(),
            record("Filesystem", "/tmp/logram_test/file_renamed removed")
        );

        fs::remove_dir(&base_path).unwrap();
        assert_eq!(
            stream.next().unwrap().unwrap(),
            record("Filesystem", "/tmp/logram_test removed")
        );
    }

}
