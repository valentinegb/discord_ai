// This is where the Discord bot code goes, the interface between the AI and the users.
// It's like the "frontend".

use dotenv::dotenv;
use std::env;
use serenity::{ client::Client, prelude::GatewayIntents };

// TODO: Gateway event handler

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let mut client = Client::builder(
        env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN env var not found"),
        GatewayIntents::empty(),
    ).await.expect("Failed to build client");

    // Print error without panicking
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
