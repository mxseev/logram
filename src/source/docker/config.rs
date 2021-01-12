use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    Local,
    Unix,
    Http,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DockerLogSourceConfig {
    pub transport: Transport,
    pub addr: String,
    pub timeout: u64,
}

impl Default for DockerLogSourceConfig {
    fn default() -> DockerLogSourceConfig {
        DockerLogSourceConfig {
            transport: Transport::Local,
            addr: String::from("unix:///var/run/docker.sock"),
            timeout: 120,
        }
    }
}
