use clap::{load_yaml, App};
use failure::Error;
use futures::{stream, Future, Stream};
use std::process;

mod config;
mod source;
use self::{
    config::Config,
    source::{FsLogSource, LogSource},
};

fn run() -> Result<(), Error> {
    let cli = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(cli).get_matches();

    let config_filename = matches.value_of("config").unwrap_or("config.yaml");
    let config = Config::read(config_filename)?;

    let fs = FsLogSource::new(config.sources.fs)?;
    let fs_stream = fs.into_stream();

    let _ = stream::empty()
        .select(fs_stream)
        .for_each(|event| {
            println!("{:?}", event);
            Ok(())
        })
        .wait();

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        process::exit(2);
    }
}
