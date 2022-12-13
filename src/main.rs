#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use serenity::{ prelude::*, async_trait, model::{ channel::Message, gateway::Ready } };
use discord_ai::{ prompt, begin_training_history, add_training_event, respond_to_training_history };
use std::env::args;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let args: Vec<String> = args().collect();

        if args.len() > 1 && &args[1] == "--training_mode" {
            println!("TRAINING MODE ACTIVE");

            return;
        }

        if msg.author.id.to_string() != dotenv!("DISCORD_APP_ID") {
            msg.channel_id.broadcast_typing(&ctx.http).await.expect("Failed to broadcast typing");

            let messages = msg.channel_id.messages(&ctx.http, |retriever| retriever.before(msg.id).limit(20)).await.expect("Failed to retrieve messages");

            let mut history: Vec<(String, String)> = Vec::new();

            for message in messages {
                history.push((message.author.name, message.content));
            }

            let response = prompt(&msg.content, &msg.author.name, history).await;

            if !response.is_empty() {
                msg.channel_id.send_message(&ctx.http, |message| message.content(response)).await.expect("Failed to send message");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("Error initializing dotenv");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(dotenv!("DISCORD_TOKEN"), intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
