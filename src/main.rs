#[macro_use]
extern crate lazy_static;

mod dice_commands;

use crate::dice_commands::dice_commands::GENERAL_GROUP;

use serenity::{
    async_trait, framework::standard::StandardFramework, model::gateway::Ready, prelude::*,
};
use std::env;

#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    token: Option<String>,
}

impl Config {
    fn new(configfile: &str) -> Self {
        let text: String = std::fs::read_to_string(configfile).unwrap();
        match serde_json::from_str(&text) {
            Ok(c) => return c,
            _ => return Config { token: None },
        };
    }
}

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

    let token: String = match config.token {
        Some(k) if k != "ENV".to_string() => k,
        _ => env::var("DISCORD_TOKEN").expect("Expected a token in the environment!"),
    };

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
