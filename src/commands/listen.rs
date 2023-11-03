use chrono::Utc;
use poise::serenity_prelude::Channel;

use crate::Context;
use crate::Error;

/// Configures the bot to listen to new messages in a given channel.
#[poise::command(slash_command, guild_only)]
pub async fn listen(
    context: Context<'_>,
    #[channel_types("Text", "News")]
    #[description = "Select the channel to monitor for messages"]
    channel: Channel,
    #[channel_types("Text", "News")]
    #[description = "Select the destination channel for the output message"]
    target: Channel,
) -> Result<(), Error> {
    let data = context.data();

    let guild_id = context.guild_id().unwrap().0 as i64; // Guild only check
    let channel_id = channel.id().0 as i64;
    let target_id = target.id().0 as i64;

    let result = sqlx::query(
        "INSERT INTO channels (id, guild_id, target_id, created_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(channel_id)
    .bind(guild_id)
    .bind(target_id)
    .bind(Utc::now().timestamp())
    .execute(&data.database)
    .await;

    match result {
        Ok(_) => {
            context
                .send(|ctx| {
                    ctx.content("This channel is now being monitored.")
                        .ephemeral(true)
                })
                .await?;
        }
        Err(why) => {
            if let Some(code) = why.as_database_error() {
                if code.is_unique_violation() {
                    context
                        .send(|ctx| {
                            ctx.content("This channel is already being monitored.")
                                .ephemeral(true)
                        })
                        .await?;
                }
            } else {
                context
                    .send(|ctx| {
                        ctx.content(format!("Failed to start monitoring this channel: {}", why))
                            .ephemeral(true)
                    })
                    .await?;
            }
        }
    }

    Ok(())
}
