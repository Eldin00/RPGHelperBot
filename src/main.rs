mod dice_commands;

use crate::dice_commands::dice_commands::GENERAL_GROUP;

use serenity::{
    async_trait,
    framework::standard::StandardFramework,
    model::gateway::Ready,
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{:?} is connected!", ready.user.name);
    }
}


#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment!");

    let framework = StandardFramework::new()
        .configure(|c| c.case_insensitivity(true).prefix("!"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Couldn't create the new client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }

}
