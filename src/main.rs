#![allow(unused_imports)]
use std::fmt::format;

use teloxide::{prelude::*, utils::command::BotCommands};

mod query;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    tracing::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Number of sales for Wymmo.com.")]
    SalesCount,
    #[command(description = "Number of rentals for Wymmo.com.")]
    RentalsCount,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::SalesCount => {
            // bot.send_message(msg.chat.id, format!("SalesCount: {}.", query_sales())).await?
            todo!()
        }
        Command::RentalsCount => {
            // bot.send_message(msg.chat.id, format!("RentalsCount: {}.", retrieve_rentals_count())).await?
            todo!()
        }
    };

    Ok(())
}
