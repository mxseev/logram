use anyhow::Result;
use bollard::{
    container::{LogOutput, LogsOptions},
    models::SystemEventsResponse,
    Docker, API_DEFAULT_VERSION,
};
use chrono::Utc;
use futures::{
    channel::mpsc::{self as futures_mpsc, SendError, Sender},
    executor, future, Future, Stream, StreamExt,
};

use crate::source::{LogRecord, LogSource, LogSourceStream};

mod config;
pub use self::config::DockerLogSourceConfig;
use self::config::Transport;

type RecordSender = Sender<Result<LogRecord>>;

#[derive(Debug, Clone)]
pub struct DockerLogSource {
    docker: Docker,
}

impl DockerLogSource {
    pub fn new(config: DockerLogSourceConfig) -> Result<Self> {
        let version = API_DEFAULT_VERSION;
        let docker = match config.transport {
            Transport::Local => Docker::connect_with_local(&config.addr, config.timeout, version)?,
            Transport::Unix => Docker::connect_with_unix(&config.addr, config.timeout, version)?,
            Transport::Http => Docker::connect_with_http(&config.addr, config.timeout, version)?,
        };

        executor::block_on(docker.info())?;

        Ok(DockerLogSource { docker })
    }
    async fn runned_containers(&self) -> Result<Vec<String>> {
        let containers = self
            .docker
            .list_containers::<String>(None)
            .await?
            .into_iter()
            .filter(|ctr| ctr.state.as_deref() == Some("running"))
            .filter_map(|ctr| ctr.names)
            .filter_map(|names| names.into_iter().next())
            .map(|name| String::from(name.trim_start_matches('/')))
            .collect();

        Ok(containers)
    }
    fn running_containers(&self) -> impl Stream<Item = Result<String>> {
        let events_stream = self.docker.events::<String>(None);

        events_stream.filter_map(|entry| {
            let resp = match entry {
                Err(error) => Some(Err(error.into())),
                Ok(event) => {
                    let typ = event.typ.as_deref();
                    let action = event.action.as_deref();

                    if typ == Some("container") && action == Some("start") {
                        container_name(event).map(Ok)
                    } else {
                        None
                    }
                }
            };

            future::ready(resp)
        })
    }
    async fn listen_runned_containers(&self, tx: RecordSender) -> Result<()> {
        let runned_containers = self.runned_containers().await?;

        for container in runned_containers {
            tokio::spawn(listen_logs(self.docker.clone(), container, tx.clone()));
        }

        Ok(())
    }
    async fn listen_running_containers(&self, tx: RecordSender) -> Result<()> {
        let mut running_containers = self.running_containers();

        while let Some(container) = running_containers.next().await {
            tokio::spawn(listen_logs(self.docker.clone(), container?, tx.clone()));
        }

        Ok(())
    }
}

impl LogSource for DockerLogSource {
    fn into_stream(self) -> LogSourceStream {
        let (mut tx, rx) = futures_mpsc::channel(10);

        let source = self.clone();
        let mut sender = tx.clone();
        tokio::spawn(async move {
            if let Err(error) = source.listen_runned_containers(sender.clone()).await {
                sender.try_send(Err(error)).unwrap();
            }
        });

        tokio::spawn(async move {
            if let Err(error) = self.listen_running_containers(tx.clone()).await {
                tx.try_send(Err(error)).unwrap();
            }
        });

        Box::pin(rx)
    }
}

fn container_name(event: SystemEventsResponse) -> Option<String> {
    event.actor?.attributes?.get("name").cloned()
}

fn entry_to_record(name: &str, body: LogOutput) -> LogRecord {
    let title = format!("{} container", name);
    let body = format!("{}", body);

    LogRecord::new(title, body)
}

fn listen_logs(
    docker: Docker,
    name: String,
    sender: Sender<Result<LogRecord>>,
) -> impl Future<Output = Result<(), SendError>> {
    let options: LogsOptions<String> = LogsOptions {
        follow: true,
        stdout: true,
        stderr: true,
        timestamps: false,
        since: Utc::now().timestamp(),
        ..Default::default()
    };

    docker
        .logs(&name, Some(options))
        .map(move |entry| match entry {
            Ok(body) => Ok(entry_to_record(&name, body)),
            Err(error) => Err(error.into()),
        })
        .map(Ok)
        .forward(sender)
}
