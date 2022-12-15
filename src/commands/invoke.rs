use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::*,
};
use super::super::openai::{ create_completion, CreateCompletionRequestBody };

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    // Get last 50 messages
    let messages = command.channel_id.messages(&ctx.http, |messages| {
        messages.limit(50)
    }).await.expect("should be able to get the last 50 messages in the current channel");

    let mut prompt = String::new();

    for message in messages {
        prompt.insert_str(0, &format!("{}: {}\n", message.author.name, message.content));
    }

    // Show "..." on Discord
    command.defer(&ctx.http).await.expect("should not error");

    // Send prompt to AI
    let completion = create_completion(CreateCompletionRequestBody {
        model: "text-davinci-003",
        prompt: Some(&format!("The following is the message history for a large language model Discord bot and some users.\n\n{prompt}\nAI-bert: ")),
        max_tokens: Some(1024),
        temperature: Some(0.9),
        user: Some(&command.user.id.to_string()),
        ..CreateCompletionRequestBody::default()
    }).await;

    if cfg!(debug_assertions) {
        dbg!(&completion);
    }

    // Respond with completion or at least stop showing "..."
    let completion_text = &completion.choices.first()
                .expect("choices should have at least one item").text;

    if completion_text.is_empty() {
        command.delete_original_interaction_response(&ctx.http).await
            .expect("interaction response should be deleted");
    } else {
        command.edit_original_interaction_response(&ctx.http, |response| {
            response.content(completion_text)
        }).await.expect("interaction response should be created");
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("invoke")
        .description("Invoke upon the mystically powers of Albert.")
}
