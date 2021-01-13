use anyhow::Result;
use log::Level;

mod logger;
pub use logger::TelegramLogger;

/// Initializes and registers the Telegram logger
pub fn init(level: Level, token: String, chat_id: String, proxy: Option<String>) -> Result<()> {
    let logger = TelegramLogger::new(level, token, chat_id, proxy)?;

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());

    Ok(())
}
