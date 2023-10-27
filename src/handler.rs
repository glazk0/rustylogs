use std::sync::Arc;

use chatgpt::prelude::Conversation;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;
use sqlx::SqlitePool;
use tokio::sync::Mutex;

pub struct Handler {
    pub database: SqlitePool,
    pub conversation: Arc<Mutex<Conversation>>,
}

// TODO: Maybe split this into events/mod.rs & separated file for each event ?

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let guild_id = match msg.guild_id {
            Some(id) => id.0 as i64,
            None => return println!("Command has not been executed in a guild"),
        };

        let channel_id = msg.channel_id.0 as i64;

        // TODO If we handle struct for Channel & Guild change this
        let row = sqlx::query_as::<_, (i64,)>(
            "SELECT target_id FROM channels WHERE guild_id = $1 AND id = $2",
        )
        .bind(guild_id)
        .bind(channel_id)
        .fetch_one(&self.database)
        .await;

        let target_id = match row {
            Ok(id) => id.0,
            Err(why) => {
                eprintln!("No channel found: {:?}", why);
                return;
            }
        };

        let channel = match ctx.cache.channel(ChannelId::from(target_id as u64)) {
            Some(channel) => channel,
            None => {
                eprintln!("Channel not found in cache");
                return;
            }
        };

        let channel = match channel.guild() {
            Some(channel) => channel,
            None => {
                println!("{} is not a guild channel", channel_id);
                return;
            }
        };

        let mut conversation = self.conversation.lock().await;

        let prompt = conversation.send_message(&msg.content).await;

        let response = match prompt {
            Ok(response) => response,
            Err(why) => {
                eprintln!("Failed to get a response from ChatGPT: {:?}", why);
                return;
            }
        };

        let message = channel
            .send_message(&ctx, |m| m.content(response.message().content.clone()))
            .await;

        match message {
            Ok(_) => println!("Successfully sent message"),
            Err(why) => eprintln!("Failed sending message: {:?}", why),
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
