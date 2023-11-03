use chrono::Utc;
use poise::serenity_prelude::{ButtonStyle, ChannelId, Context, Message, SerenityError};

use crate::Data;

pub async fn message(
    new_message: &Message,
    context: &Context,
    data: &Data,
) -> Result<(), SerenityError> {
    if new_message.author.bot {
        return Ok(());
    }

    let guild_id = new_message.guild_id.expect("Failed to fetch guild id").0 as i64;

    sqlx::query("INSERT OR IGNORE INTO guilds (id, created_at) VALUES (?, ?)")
        .bind(guild_id)
        .bind(Utc::now().timestamp())
        .execute(&data.database)
        .await
        .expect("Failed to insert guild");

    let channel_id = new_message.channel_id.0 as i64;

    let row =
        sqlx::query_as::<_, (i64,)>("SELECT target_id FROM channels WHERE guild_id = ? AND id = ?")
            .bind(guild_id)
            .bind(channel_id)
            .fetch_optional(&data.database)
            .await
            .expect("Failed to fetch channel target");

    let target_id = match row {
        Some(row) => row.0,
        None => return Ok(()),
    };

    let channel = ChannelId(target_id as u64)
        .to_channel(&context)
        .await
        .expect("Failed to fetch channel");

    let channel = match channel.guild() {
        Some(channel) => channel,
        None => return Ok(()),
    };

    let mut conversation = data.conversation.lock().await;

    let response = conversation
        .send_message(&new_message.content)
        .await
        .expect("Failed to send message to GPT");

    let message = channel
        .send_message(&context, |message| {
            message
                .content(&response.message().content)
                .components(|c| {
                    c.create_action_row(|a| {
                        a.create_button(|b| {
                            b.custom_id("send").label("Send").style(ButtonStyle::Danger)
                        })
                    })
                })
        })
        .await
        .expect("Failed to send message to Discord");

    sqlx::query("INSERT INTO messages (id, guild_id, message_id, created_at) VALUES (?, ?, ?, ?)")
        .bind(new_message.id.0 as i64)
        .bind(guild_id)
        .bind(message.id.0 as i64)
        .bind(new_message.timestamp.timestamp())
        .execute(&data.database)
        .await
        .expect("Failed to insert message into database");

    Ok(())
}
