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
    fn into_message(self) -> String {
        String::from("todo: format message")
    }
}
