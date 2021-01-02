use anyhow::Result;
use clap::ArgMatches;
use futures::StreamExt;

mod args;
mod config;
mod source;
mod telegram;

use config::Config;
use telegram::{Telegram, TelegramConfig};

async fn run_echo_id(matches: &ArgMatches<'_>) -> Result<()> {
    let token = matches.value_of("token").map(String::from).unwrap();
    let proxy = matches.value_of("proxy").map(String::from);

    let config = TelegramConfig::for_echo_id(token, proxy);
    let telegram = Telegram::new(config)?;

    telegram.echo_id().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = args::clap_app().get_matches();
    if let Some(echo_id_args) = matches.subcommand_matches("echo_id") {
        run_echo_id(echo_id_args).await?;

        return Ok(());
    }

    let config_path = matches.value_of("config").unwrap();
    let config = Config::from_file(config_path)?;

    let telegram = Telegram::new(config.telegram)?;
    let mut sources_stream = source::init_log_sources(config.sources)?;

    if config.hello_message {
        telegram.send_hello().await?;
    }

    while let Some(result) = sources_stream.next().await {
        match result {
            Ok(record) => telegram.send_record(record).await?,
            Err(error) => {
                eprintln!("{:?}", error);
                telegram.send_error(error).await?;
            }
        }
    }

    Ok(())
}
