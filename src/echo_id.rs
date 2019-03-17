use failure::{err_msg, Error};
use futures::Future;

use crate::telegram::{types::Update, Telegram};

fn print_updates(updates: Vec<Update>) -> Result<(), Error> {
    if updates.len() == 0 {
        println!("Updates not found. Send any message to bot and try again");
    } else {
        for update in updates {
            let chat = update.message.chat;
            let from = chat.username.unwrap_or(String::from("<unknown>"));

            println!(
                "Received message from @{} in chat with ID: {}",
                from, chat.id
            );
        }
    }

    Ok(())
}

pub fn echo_id(token: Option<&str>) -> Result<(), Error> {
    let token = token.ok_or_else(|| err_msg("cli parse error"))?;
    let fut = Telegram::get_updates(token)
        .and_then(print_updates)
        .map_err(|error| eprintln!("Telegram error: {}", error));

    tokio::run(fut);
    Ok(())
}
