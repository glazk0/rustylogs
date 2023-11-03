use poise::serenity_prelude::{Context, Error, Guild, UnavailableGuild};

use crate::Data;

pub async fn guild_delete(
    incomplete: &UnavailableGuild,
    full: &Option<Guild>,
    _context: &Context,
    data: &Data,
) -> Result<(), Error> {
    if let Some(guild) = full {
        let guild_id = guild.id.0 as i64;

        let result = sqlx::query("DELETE FROM guilds WHERE id = ?")
            .bind(guild_id)
            .execute(&data.database)
            .await;

        match result {
            Ok(_) => println!("Successfully deleted guild {}", guild_id),
            Err(why) => eprintln!("Failed to delete guild {}: {}", guild_id, why),
        }
    } else {
        let guild_id = incomplete.id.0 as i64;

        let result = sqlx::query("DELETE FROM guilds WHERE id = ?")
            .bind(guild_id)
            .execute(&data.database)
            .await;

        match result {
            Ok(_) => println!("Successfully deleted guild {}", guild_id),
            Err(why) => eprintln!("Failed to delete guild {}: {}", guild_id, why),
        }
    }

    Ok(())
}
