use failure::Error;
use futures::Stream;

mod fs;
mod journald;
pub use self::{fs::FsLogSource, journald::JournaldLogSource};

#[derive(Debug)]
pub struct LogRecord {
    pub title: String,
    pub body: Option<String>,
}

#[derive(Debug)]
pub enum LogSourceEvent {
    Record(LogRecord),
    Error(Error),
}

pub type LogSourceStream = Stream<Item = LogSourceEvent, Error = ()> + Send;

pub trait LogSource {
    fn into_stream(self) -> Box<LogSourceStream>;
}
