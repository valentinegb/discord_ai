// This is where the Discord bot code goes, the interface between the AI and the users.
// It's like the "frontend".

use dotenv::dotenv;
use std::env;
use serenity::{
    client::Client,
    prelude::*,
    async_trait,
    model::{
        gateway::Ready,
        id::GuildId,
        application::{
            command::Command,
            interaction::Interaction,
        },
    },
};

mod commands;

struct Handler;

const DEBUG_GUILD: &str = "1051651636382142504";

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "invoke" => commands::invoke::run(ctx, command).await,
                _ => (),
            }
        }
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("{} is online", data_about_bot.user.name);

        if cfg!(debug_assertions) {
            let debug_guild = GuildId(DEBUG_GUILD.parse().expect("Could not parse DEBUG_GUILD"));

            debug_guild.set_application_commands(ctx.http, |commands| {
                commands.create_application_command(|command| commands::invoke::register(command))
            }).await.expect("Failed to set guild commands");
        } else {
            Command::set_global_application_commands(ctx.http, |commands| {
                commands.create_application_command(|command| commands::invoke::register(command))
            }).await.expect("Failed to set global commands");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let mut client = Client::builder(
        env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN env var not found"),
        GatewayIntents::empty(),
    )
        .event_handler(Handler)
        .await
        .expect("Failed to build client");

    // Print error without panicking
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
