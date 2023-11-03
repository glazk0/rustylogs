use std::sync::Arc;
use std::time::Duration;

use chatgpt::prelude::{ChatGPT, ModelConfiguration};

use poise::serenity_prelude::{self as serenity, Context, Error, Ready};
use poise::{Event, FrameworkContext};

use tokio::sync::Mutex;

use crate::config::Config;
use crate::Data;

use guild_create::*;
use guild_delete::*;
use interaction_create::*;
use message::*;
use message_delete::*;

mod guild_create;
mod guild_delete;
mod interaction_create;
mod message;
mod message_delete;

pub async fn listen(
    context: &Context,
    event: &Event<'_>,
    _framework: FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Message { new_message } => message(new_message, context, data).await,
        Event::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id,
        } => message_delete(channel_id, deleted_message_id, guild_id, context, data).await,
        Event::GuildCreate { guild, is_new } => guild_create(guild, is_new, context, data).await,
        Event::GuildDelete { incomplete, full } => {
            guild_delete(incomplete, full, context, data).await
        }
        Event::InteractionCreate { interaction } => {
            interaction_create(interaction, context, data).await
        }
        _ => Ok(()),
    }
}

pub async fn on_bot_ready(
    context: &serenity::Context,
    ready: &Ready,
    framework: &poise::Framework<Data, Error>,
) -> Result<Data, Error> {
    let config = Config::load().expect("Failed to load configuration file");

    println!(
        "Logged in as {}#{}",
        ready.user.name, ready.user.discriminator
    );

    let builder = poise::builtins::create_application_commands(&framework.options().commands);

    serenity::Command::set_global_application_commands(&context.http, |commands| {
        *commands = builder;
        commands
    })
    .await?;

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true)
                .foreign_keys(true),
        )
        .await
        .expect("Couldn't connect to the database");

    if let Err(why) = sqlx::migrate!("./migrations").run(&database).await {
        println!("Couldn't run database migrations: {:?}", why);
    }

    let gpt = ChatGPT::new_with_config(
        &config.key.to_owned(),
        ModelConfiguration {
            timeout: Duration::from_secs(360),
            ..Default::default()
        },
    )
    .expect("Failed to create ChatGPT client");

    let conversation = Arc::new(Mutex::new(gpt.new_conversation_directed(&config.prompt)));

    Ok(Data {
        conversation,
        database,
    })
}
