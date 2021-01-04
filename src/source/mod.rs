use failure::Error;
use futures::Stream;

mod fs;
#[cfg(feature = "journald-source")]
mod journald;

pub use self::fs::FsLogSource;

#[cfg(feature = "journald-source")]
pub use self::journald::JournaldLogSource;

#[derive(Debug, PartialEq)]
pub struct LogRecord {
    pub title: String,
    pub body: String,
}

pub type LogSourceStream = Stream<Item = LogRecord, Error = Error> + Send;

pub trait LogSource {
    fn into_stream(self) -> Box<LogSourceStream>;
}
