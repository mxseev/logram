use failure::Error;
use futures::{Future, Stream};
use tgbot::{methods::SendMessage, Api as TelegramApi, UpdatesStream};
use tokio;

pub fn echo_id(token: &str) -> Result<(), Error> {
    let telegram = TelegramApi::new::<&str, String>(token, None)?;
    let updates = UpdatesStream::new(telegram.clone());

    let fut = updates
        .filter_map(|update| update.get_chat_id())
        .map(|chat_id| {
            let text = format!("Chat id: {}", chat_id);
            SendMessage::new(chat_id, text)
        })
        .for_each(move |method| telegram.execute(&method).map(|_| ()))
        .map_err(|error| eprintln!("Error: {}", error));

    tokio::run(fut);

    Ok(())
}
