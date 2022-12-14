use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::interaction::application_command::ApplicationCommandInteraction,
        application::command::CommandOptionType,
    },
    prelude::*,
};

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    command.create_interaction_response(ctx.http, |response| {
        response.interaction_response_data(|message| {
            for option in &command.data.options {
                if option.name == "prompt" {
                    let prompt = option.value.as_ref().expect("Expected value").as_str().expect("Expected string");

                    return message.content(prompt);
                }
            }

            message.content("Error")
        })
    })
        .await.expect("Failed to create interaction response");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("invoke")
        .description("Placeholder description")
        .create_option(|option| {
            option
                .kind(CommandOptionType::String)
                .name("prompt")
                .description("Placeholder description")
                .required(true)
                .max_length(1024)
        })
}
