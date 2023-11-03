use poise::serenity_prelude::{ChannelId, Context, GuildId, MessageId, SerenityError};

use crate::Data;

pub async fn message_delete(
    channel_id: &ChannelId,
    deleted_message_id: &MessageId,
    _guild_id: &Option<GuildId>,
    context: &Context,
    data: &Data,
) -> Result<(), SerenityError> {
    let row = sqlx::query_as::<_, (i64,)>("SELECT message_id FROM messages WHERE id = ?")
        .bind(deleted_message_id.0 as i64)
        .fetch_optional(&data.database)
        .await
        .expect("Failed to fetch message id");

    let message_id = match row {
        Some(row) => MessageId(row.0 as u64),
        None => return Ok(()),
    };

    let channel = channel_id
        .to_channel(&context)
        .await
        .expect("Failed to fetch channel");

    let channel = match channel.guild() {
        Some(channel) => channel,
        None => return Ok(()),
    };

    channel
        .delete_messages(&context, vec![message_id])
        .await
        .expect("Failed to delete message");

    sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(message_id.0 as i64)
        .execute(&data.database)
        .await
        .expect("Failed to delete message");

    Ok(())
}
