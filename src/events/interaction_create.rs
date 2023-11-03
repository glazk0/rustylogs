use poise::serenity_prelude::{Context, Error, Interaction, InteractionResponseType};

use crate::Data;

pub async fn interaction_create(
    interaction: &Interaction,
    context: &Context,
    data: &Data,
) -> Result<(), Error> {
    let http = &context.http;

    match interaction {
        Interaction::MessageComponent(command) => match command.data.custom_id.as_str() {
            "adjust" => {
                let mut message = http
                    .get_message(command.channel_id.0, command.message.id.0)
                    .await?;

                let content = message.content.clone();

                let mut conversation = data.conversation.lock().await;

                command
                    .create_interaction_response(http, |response| {
                        response.kind(InteractionResponseType::DeferredUpdateMessage)
                    })
                    .await
                    .ok();

                let response = conversation
                    .send_message(&content)
                    .await
                    .expect("Failed to send message to GPT");

                message
                    .edit(http, |message| message.content(&response.message().content))
                    .await?;
            }
            "send" => {
                // TODO

                command
                    .create_interaction_response(http, |response| {
                        response.kind(InteractionResponseType::DeferredUpdateMessage)
                    })
                    .await
                    .ok();
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}
