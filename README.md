# logram

Utility that takes logs from anywhere and sends them to Telegram. Supports log collection from files, journald and docker containers.

## Usage

WIP

## Usage with `log`
Logram has an implementation of [`log::Logger`](https://crates.io/crates/log).  

Import logram as library:
```toml
[dependencies]
logram = "2.0"
```

Initialize the logram:
```rust
use logram;
use log::{warn, info, Level};

fn main() {
    logram::init(
        Level::Error, // log level
        String::from("1496993932:AAFrF5aTnQEeruljp3ZHqVUSkgVS9Ra_aT8"), // bot token
        String::from("79098882"), // chat ID
        None, // proxy url, if needed
    )
    .unwrap();

    info!("Application started");

    if cfg!(target_os = "windows") {
        warn!("Oh, shi...");
    }
}
```

Or you can use the logram's `log::Logger` implementation itself in the composite logger, e.g. [multi_log](https://crates.io/crates/multi_log):
```rust
use log::{debug, error, info, trace, warn, Level, LevelFilter};
use logram::TelegramLogger;
use simplelog::{self, SimpleLogger};
use multi_log::MultiLogger;

fn main() {
    let simple_logger = SimpleLogger::new(LevelFilter::Warn, simplelog::Config::default());
    let logram = TelegramLogger::new(
        Level::Info,
        String::from("1496993932:AAFrF5aTnQEeruljp3ZHqVUSkgVS9Ra_aT8"),
        String::from("79098882"),
        None,
    )
    .unwrap();

    MultiLogger::init(vec![simple_logger, Box::new(logram)], Level::Info).unwrap();

    debug!("some debug message");
    error!("well, I'm a useless example of code, sad");
}
```

Warning: log records with target starts with `tokio_reactor`, `hyper`, `mio`, `want` or `reqwest` will be skipped, because [limitations in log](https://github.com/rust-lang/log/issues/312).

