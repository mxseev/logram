use anyhow::Result;
use cfg_if::cfg_if;
use futures::stream::{self, Stream};
use std::pin::Pin;

mod config;
mod record;
pub use config::LogSourcesConfig;
pub use record::LogRecord;

cfg_if! { if #[cfg(feature = "ls_counter")] {
    pub mod counter;
    use counter::CounterLogSource;
}}
cfg_if! { if #[cfg(feature = "ls_filesystem")] {
    pub mod filesystem;
    use filesystem::FilesystemLogSource;
}}
cfg_if! { if #[cfg(feature = "ls_journald")] {
    pub mod journald;
    use journald::JournaldLogSource;
}}
cfg_if! { if #[cfg(feature = "ls_docker")] {
    pub mod docker;
    use docker::DockerLogSource;
}}

pub type LogSourceStream = Pin<Box<dyn Stream<Item = Result<LogRecord>>>>;

pub trait LogSource {
    fn into_stream(self) -> LogSourceStream;
}

pub fn init_log_sources(config: LogSourcesConfig) -> Result<LogSourceStream> {
    let mut streams: Vec<LogSourceStream> = Vec::new();

    #[cfg(feature = "ls_counter")]
    if config.counter.enabled {
        let counter = CounterLogSource::new(config.counter.inner);
        streams.push(counter.into_stream());
    }

    #[cfg(feature = "ls_filesystem")]
    if config.filesystem.enabled {
        let filesystem = FilesystemLogSource::new(config.filesystem.inner)?;
        streams.push(filesystem.into_stream());
    }

    #[cfg(feature = "ls_journald")]
    if config.journald.enabled {
        let filesystem = JournaldLogSource::new(config.journald.inner)?;
        streams.push(filesystem.into_stream());
    }

    #[cfg(feature = "ls_docker")]
    if config.docker.enabled {
        let filesystem = DockerLogSource::new(config.docker.inner)?;
        streams.push(filesystem.into_stream());
    }

    Ok(Box::pin(stream::select_all(streams)))
}
