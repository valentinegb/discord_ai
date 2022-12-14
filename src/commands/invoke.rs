use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::interaction::application_command::ApplicationCommandInteraction,
        application::command::CommandOptionType,
    },
    prelude::*,
};

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    // Show "..." on Discord
    command.defer(&ctx.http).await.expect("should not error");

    // Send prompt to AI
    todo!();

    // Respond with completion
    todo!();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("invoke")
        .description("Invoke upon the mystically powers of Albert.")
        .create_option(|option| {
            option
                .kind(CommandOptionType::String)
                .name("prompt")
                .description("What is it you wish you know?")
                .required(true)
                .max_length(1024)
        })
}
