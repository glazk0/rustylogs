use poise::serenity_prelude::Channel;

use crate::Context;
use crate::Error;

/// Stops the bot from listening to new messages in a given channel.
#[poise::command(slash_command, guild_only)]
pub async fn stop(
    context: Context<'_>,
    #[channel_types("Text", "News")]
    #[description = "Select the channel to stop monitoring for messages"]
    channel: Channel,
) -> Result<(), Error> {
    let data = context.data();

    let guild_id = context.guild_id().unwrap().0 as i64; // Guild only check
    let channel_id = channel.id().0 as i64;

    let row = sqlx::query_as::<_, (i64,)>("SELECT id FROM channels WHERE id = ? AND guild_id = ?")
        .bind(channel_id)
        .bind(guild_id)
        .fetch_optional(&data.database)
        .await
        .expect("Failed to fetch channel id");

    if row.is_none() {
        context
            .send(|ctx| {
                ctx.content("This channel is not being monitored.")
                    .ephemeral(true)
            })
            .await?;

        return Ok(());
    }

    let result = sqlx::query("DELETE FROM channels WHERE id = ? AND guild_id = ?")
        .bind(channel_id)
        .bind(guild_id)
        .execute(&data.database)
        .await;

    match result {
        Ok(_) => {
            context
                .send(|ctx| {
                    ctx.content("This channel is no longer being monitored.")
                        .ephemeral(true)
                })
                .await?;
        }
        Err(why) => {
            context
                .send(|ctx| {
                    ctx.content(format!("Failed to stop monitoring this channel: {}", why))
                        .ephemeral(true)
                })
                .await?;
        }
    }

    Ok(())
}
