use serde::Deserialize;

use super::counter::CounterLogSourceConfig as CounterConfig;
use super::docker::DockerLogSourceConfig as DockerConfig;
use super::filesystem::FilesystemLogSourceConfig as FilesystemConfig;
use super::journald::JournaldLogSourceConfig as JournaldConfig;

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
    pub counter: LogSourceConfig<CounterConfig>,
    pub filesystem: LogSourceConfig<FilesystemConfig>,
    pub journald: LogSourceConfig<JournaldConfig>,
    pub docker: LogSourceConfig<DockerConfig>,
}
