use std::path::PathBuf;

use crate::source::LogRecord;

#[derive(Debug)]
pub enum FsEvent {
    Created { path: PathBuf },
    Writed { path: PathBuf, new_content: String },
    Removed { path: PathBuf },
    Renamed { from: PathBuf, to: PathBuf },
}

impl FsEvent {
    pub fn into_record(self) -> LogRecord {
        let (title, body) = match self {
            FsEvent::Created { path } => {
                let title = String::from("Filesystem");
                let body = format!("{} created", path.display());

                (title, body)
            }
            FsEvent::Writed { path, new_content } => {
                let title = path.display().to_string();
                let body = new_content;

                (title, body)
            }
            FsEvent::Removed { path } => {
                let title = String::from("Filesystem");
                let body = format!("{} removed", path.display());

                (title, body)
            }
            FsEvent::Renamed { from, to } => {
                let title = String::from("Filesystem");
                let body = format!("{} renamed to {}", from.display(), to.display());

                (title, body)
            }
        };

        LogRecord { title, body }
    }
}
