#[macro_use]
extern crate lazy_static;

mod dice_commands;
mod config;

use crate::dice_commands::dice_commands::GENERAL_GROUP;
use crate::config::config::Config;

use serenity::{
    async_trait, framework::standard::StandardFramework, model::gateway::Ready, prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{:?} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let config = Config::new("config.json");

    let token: String = config.get_token();

    let framework = StandardFramework::new()
        .configure(|c| 
            c.case_insensitivity(true)
            .prefix(config.get_prefix())
        )
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
