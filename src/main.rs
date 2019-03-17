#![recursion_limit = "128"]

use clap::{crate_version, load_yaml, App};
use failure::{err_msg, Error};
use futures::{future::Either, stream, Future, Stream};
use std::process;
use tokio;

mod config;
mod source;
mod telegram;
mod utils;
use self::{
    config::Config,
    source::{FsLogSource, JournaldLogSource, LogSource},
    telegram::Telegram,
};

fn echo_id(token: Option<&str>) -> Result<(), Error> {
    let token = token.ok_or_else(|| err_msg("cli parse error"))?;
    let future = Telegram::echo_id(token).map_err(|error| eprintln!("Telegram error: {}", error));

    tokio::run(future);
    Ok(())
}

fn run() -> Result<(), Error> {
    let cli = load_yaml!("../cli.yaml");
    let app = App::from_yaml(cli).version(crate_version!());
    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("echo_id") {
        let token = matches.value_of("token");
        return echo_id(token);
    }

    let config_filename = matches.value_of("config").unwrap_or("config.yaml");
    let config = Config::read(config_filename)?;

    let telegram = Telegram::new(config.telegram)?;

    let fs = FsLogSource::new(config.sources.fs)?;
    let fs_stream = fs.into_stream();

    let journald = JournaldLogSource::new(config.sources.journald)?;
    let journald_stream = journald.into_stream();

    let main_loop = stream::empty()
        .select(fs_stream)
        .select(journald_stream)
        .then(move |result| match result {
            Ok(record) => Either::A(telegram.send_record(record)),
            Err(error) => Either::B(telegram.send_error(error)),
        })
        .for_each(|_| Ok(()))
        .map_err(|error| eprintln!("Telegram error: {}", error));

    tokio::run(main_loop);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        process::exit(2);
    }
}
