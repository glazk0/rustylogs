use chrono::Utc;
use poise::serenity_prelude::{Context, Error, Guild};

use crate::Data;

pub async fn guild_create(
    guild: &Guild,
    is_new: &bool,
    _context: &Context,
    data: &Data,
) -> Result<(), Error> {
    if !*is_new {
        return Ok(());
    }

    let guild_id = guild.id.0 as i64;

    let result = sqlx::query("INSERT OR IGNORE INTO guilds (id, created_at) VALUES (?, ?)")
        .bind(guild_id)
        .bind(Utc::now().timestamp())
        .execute(&data.database)
        .await;

    match result {
        Ok(_) => println!("Successfully inserted guild {}", guild_id),
        Err(why) => println!("Failed to insert guild: {}", why),
    }

    Ok(())
}
