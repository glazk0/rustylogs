use crate::Context;
use crate::Error;

/// Adjusts the bot's prompt without restarting the bot.
#[poise::command(slash_command, guild_only)]
pub async fn adjust(
    context: Context<'_>,
    #[description = "The content of the prompt"] content: String,
) -> Result<(), Error> {
    let data = context.data();

    let mut conversation = data.conversation.lock().await;

    context.defer_ephemeral().await?;

    let prompt = format!(
        "{}\n{}",
        content, "Confirm the prompt and wait for other messages to be generated."
    );

    // we set the prompt
    if let Err(why) = conversation.send_message(prompt).await {
        context
            .send(|c| c.content(format!("Failed to adjust the prompt: {}", why)))
            .await?;
    }

    context
        .send(|c| c.content("Prompt adjusted successfully."))
        .await?;

    Ok(())
}
