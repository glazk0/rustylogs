use std::sync::Arc;

use chatgpt::prelude::Conversation;
use dotenvy::dotenv;
use poise::serenity_prelude::{self as serenity, Error};
use poise::FrameworkOptions;
use sqlx::SqlitePool;
use tokio::sync::Mutex;

use commands::{adjust, list, listen, stop};

mod commands;
mod config;
mod events;

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    conversation: Arc<Mutex<Conversation>>,
    database: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let token = std::env::var("TOKEN").expect("TOKEN is missing in the .env");

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = poise::Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                listen::listen(),
                adjust::adjust(),
                stop::stop(),
                list::list(),
            ],
            event_handler: |context, event, framework, data| {
                Box::pin(events::listen(context, event, framework, data))
            },
            ..FrameworkOptions::default()
        })
        .token(token)
        .intents(intents)
        .setup(|context, ready, framework| {
            Box::pin(events::on_bot_ready(context, ready, framework))
        })
        .build()
        .await
        .expect("Client initialization failed");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
