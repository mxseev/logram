use serde::Deserialize;

#[cfg(feature = "ls_counter")]
use super::counter::CounterLogSourceConfig as CounterConfig;

#[cfg(feature = "ls_filesystem")]
use super::filesystem::FilesystemLogSourceConfig as FilesystemConfig;

#[cfg(feature = "ls_journald")]
use super::journald::JournaldLogSourceConfig as JournaldConfig;

#[cfg(feature = "ls_docker")]
use super::docker::DockerLogSourceConfig as DockerConfig;

fn default_enabled() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct LogSourceConfig<T> {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(flatten)]
    pub inner: T,
}

impl<T: Default> Default for LogSourceConfig<T> {
    fn default() -> Self {
        LogSourceConfig {
            enabled: false,
            inner: T::default(),
        }
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct LogSourcesConfig {
    #[cfg(feature = "ls_counter")]
    pub counter: LogSourceConfig<CounterConfig>,
    #[cfg(feature = "ls_filesystem")]
    pub filesystem: LogSourceConfig<FilesystemConfig>,
    #[cfg(feature = "ls_journald")]
    pub journald: LogSourceConfig<JournaldConfig>,
    #[cfg(feature = "ls_docker")]
    pub docker: LogSourceConfig<DockerConfig>,
}
