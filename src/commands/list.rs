use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;

use crate::Context;
use crate::Error;

/// List all channels being monitored by the bot.
#[poise::command(slash_command, guild_only)]
pub async fn list(context: Context<'_>) -> Result<(), Error> {
    let data = context.data();

    let guild_id = context.guild_id().unwrap().0 as i64; // Guild only check

    let channels = sqlx::query_as::<_, (i64, i64, i64)>(
        "SELECT id, target_id, created_at FROM channels WHERE guild_id = ?",
    )
    .bind(guild_id)
    .fetch_all(&data.database)
    .await;

    match channels {
        Ok(channels) => {
            if channels.is_empty() {
                context
                    .send(|ctx| {
                        ctx.content("There are no channels being monitored.")
                            .ephemeral(true)
                    })
                    .await?;

                return Ok(());
            }

            let mut message = String::new();

            for (channel_id, target_id, created_at) in channels {
                let date = DateTime::<Utc>::from_naive_utc_and_offset(
                    NaiveDateTime::from_timestamp_opt(created_at, 0).unwrap(),
                    Utc,
                );

                message.push_str(&format!(
                    "<#{}> -> <#{}> (since {})\n",
                    channel_id,
                    target_id,
                    date.format("%Y-%m-%d %H:%M:%S")
                ));
            }

            context
                .send(|ctx| ctx.content(message).ephemeral(true))
                .await?;
        }
        Err(why) => {
            context
                .send(|ctx| {
                    ctx.content(format!("Failed to fetch channels: {}", why))
                        .ephemeral(true)
                })
                .await?;
        }
    }

    Ok(())
}
