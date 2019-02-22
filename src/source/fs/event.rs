use std::path::PathBuf;

use crate::source::LogRecord;

#[derive(Debug)]
pub enum FsEvent {
    Created { path: PathBuf },
    Writed { path: PathBuf, new_content: String },
    Removed { path: PathBuf },
    Renamed { from: PathBuf, to: PathBuf },
}

impl Into<LogRecord> for FsEvent {
    fn into(self) -> LogRecord {
        let (title, body) = match self {
            FsEvent::Created { path } => {
                let title = format!("{} was created", path.display());
                (title, None)
            }
            FsEvent::Writed { path, new_content } => {
                let title = path.display().to_string();
                let body = new_content;

                (title, Some(body))
            }
            FsEvent::Removed { path } => {
                let title = format!("{} was removed", path.display());
                (title, None)
            }
            FsEvent::Renamed { from, to } => {
                let title = format!("{} renamed to {}", from.display(), to.display());
                (title, None)
            }
        };

        LogRecord { title, body }
    }
}
