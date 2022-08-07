use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    Client,
};
use std::env;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&context.http, "Pong!").await {
                println!("Error sending message: {}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
 
#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment!");
    let mut client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Couldn't create the new client!");
    if let Err(why) = client.start().await {
        println!("Client error: {}", why)
    };
}
