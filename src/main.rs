#![allow(unused_imports)]
use std::fmt::format;

use teloxide::{prelude::*, utils::command::BotCommands};

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

struct Query;

#[juniper::graphql_object]
impl Query {
    fn accurate_counters() -> Counters {
        Counters {
            sales: retrieve_sales_count(),
            rentals: retrieve_rentals_count(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
struct Counters {
    sales: i32,
    rentals: i32,
}

fn retrieve_sales_count() -> i32 {
    let query = r#"
    query {
        sales(query:"")
        }
    "#;
}

fn retrieve_rentals_count() -> i32 {
    let query = r#"
    query {
        rentals(query:"")
        }
    "#;
}


async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::SalesCount => {
            bot.send_message(msg.chat.id, format!("SalesCount: {}.", retrieve_sales_count())).await?
        }
        Command::RentalsCount => {
            bot.send_message(msg.chat.id, format!("RentalsCount: {}.", retrieve_rentals_count())).await?
        }
    };

    Ok(())
}
