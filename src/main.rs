use std::{collections::HashSet, sync::Arc, time::Duration};

use chatgpt::prelude::{ChatGPT, ModelConfiguration};
use serenity::{framework::StandardFramework, http::Http, prelude::GatewayIntents, Client};
use tokio::sync::Mutex;

mod config;
mod handler;

use config::Config;
use handler::Handler;

#[tokio::main]
async fn main() {
    let config = Config::load().expect("Failed to load configuration file");

    if config.discord.token.is_empty() {
        panic!("You should configure your config.toml");
    }

    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to the database");

    if let Err(why) = sqlx::migrate!("./migrations").run(&database).await {
        println!("Couldn't run database migrations: {:?}", why);
    }

    let gpt = ChatGPT::new_with_config(
        &config.openai.api_key.to_owned(),
        ModelConfiguration {
            timeout: Duration::from_secs(60),
            ..Default::default()
        },
    )
    .expect("Failed to create GPT client");

    let conversation = Arc::new(Mutex::new(
        gpt.new_conversation_directed(&config.openai.prompt),
    ));

    let handler = Handler {
        database,
        conversation,
    };

    let http = Http::new(&config.discord.token);

    let owners = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }

            owners
        }
        Err(why) => panic!("Could not access the application info {:?}", why),
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework =
        StandardFramework::new().configure(|c| c.allow_dm(false).ignore_bots(true).owners(owners));

    let mut client = Client::builder(&config.discord.token.to_owned(), intents)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
