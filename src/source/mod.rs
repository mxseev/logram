use failure::Error;
use futures::Stream;

mod fs;
mod journald;
pub use self::{fs::FsLogSource, journald::JournaldLogSource};

#[derive(Debug, PartialEq)]
pub struct LogRecord {
    pub title: String,
    pub body: String,
}

pub type LogSourceStream = Stream<Item = LogRecord, Error = Error> + Send;

pub trait LogSource {
    fn into_stream(self) -> Box<LogSourceStream>;
}
