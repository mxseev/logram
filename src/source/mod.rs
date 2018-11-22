use failure::Error;
use futures::Stream;
use std::fmt::Debug;

mod fs;
pub use self::fs::FsLogSource;

pub trait LogRecord: Debug + Send {
    fn into_message(self) -> String;
}

#[derive(Debug)]
pub enum LogSourceEvent {
    Record(Box<LogRecord>),
    Error(Error),
}

pub type LogSourceStream = Stream<Item = LogSourceEvent, Error = ()>;

pub trait LogSource {
    fn into_stream(self) -> Box<LogSourceStream>;
}
