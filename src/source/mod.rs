use std::pin::Pin;

use anyhow::Result;
use futures::stream::{self, Stream};

mod config;
mod record;

pub mod counter;
pub mod docker;
pub mod filesystem;
pub mod journald;

pub use config::LogSourcesConfig;
pub use record::LogRecord;

use counter::CounterLogSource;
use docker::DockerLogSource;
use filesystem::FilesystemLogSource;
use journald::JournaldLogSource;

pub type LogSourceStream = Pin<Box<dyn Stream<Item = Result<LogRecord>>>>;

pub trait LogSource {
    fn into_stream(self) -> LogSourceStream;
}

pub fn init_log_sources(config: LogSourcesConfig) -> Result<LogSourceStream> {
    let mut streams = Vec::new();

    if config.counter.enabled {
        let counter = CounterLogSource::new(config.counter.inner);
        streams.push(counter.into_stream());
    }

    if config.filesystem.enabled {
        let filesystem = FilesystemLogSource::new(config.filesystem.inner)?;
        streams.push(filesystem.into_stream());
    }

    if config.journald.enabled {
        let filesystem = JournaldLogSource::new(config.journald.inner)?;
        streams.push(filesystem.into_stream());
    }

    if config.docker.enabled {
        let filesystem = DockerLogSource::new(config.docker.inner)?;
        streams.push(filesystem.into_stream());
    }

    Ok(Box::pin(stream::select_all(streams)))
}
