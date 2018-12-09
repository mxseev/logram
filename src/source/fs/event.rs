use std::path::PathBuf;

use crate::source::LogRecord;

#[derive(Debug)]
pub enum FsEvent {
    Created { path: PathBuf },
    Writed { path: PathBuf, new_content: String },
    Removed { path: PathBuf },
    Renamed { from: PathBuf, to: PathBuf },
}

impl LogRecord for FsEvent {
    fn to_message(&self) -> String {
        match self {
            FsEvent::Created { path } => format!("*{}* was created", path.display()),
            FsEvent::Writed { path, new_content } => {
                format!("*{}*```\n{}```", path.display(), new_content)
            }
            FsEvent::Removed { path } => format!("*{}* was removed", path.display()),
            FsEvent::Renamed { from, to } => {
                format!("*{}* renamed to *{}*", from.display(), to.display())
            }
        }
    }
}
