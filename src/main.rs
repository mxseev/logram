use clap::{load_yaml, App};
use failure::Error;
use futures::{stream, Future, Stream};
use std::process;
use tokio;

mod config;
mod source;
mod telegram;
use self::{
    config::Config,
    source::{FsLogSource, JournaldLogSource, LogSource},
    telegram::Telegram,
};

fn run() -> Result<(), Error> {
    let cli = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(cli).get_matches();

    if let Some(matches) = matches.subcommand_matches("echo_id") {
        let token = matches.value_of("token").unwrap();
        Telegram::echo_id(token)?;

        return Ok(());
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
        .map(|event| event.to_message())
        .for_each(move |message| {
            telegram
                .send(&message)
                .map_err(|error| println!("Error: {}", error))
        });

    tokio::run(main_loop);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        process::exit(2);
    }
}
