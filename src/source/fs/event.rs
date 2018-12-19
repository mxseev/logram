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
                let body = format!("{} was created", path.display());
                (None, body)
            }
            FsEvent::Writed { path, new_content } => {
                let title = path.display().to_string();
                (Some(title), new_content)
            }
            FsEvent::Removed { path } => {
                let body = format!("{} was removed", path.display());
                (None, body)
            }
            FsEvent::Renamed { from, to } => {
                let body = format!("{} renamed to {}", from.display(), to.display());
                (None, body)
            }
        };

        LogRecord { title, body }
    }
}
