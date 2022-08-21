#[macro_use]
extern crate lazy_static;

mod config;
mod dice_commands;

use crate::config::config::Config;
use crate::dice_commands::dice_commands::GENERAL_GROUP;

use clap::Parser;
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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Discord token
    #[clap(short, long, value_parser)]
    token: Option<String>,
    /// Command prefix to use
    #[clap(short, long, value_parser)]
    prefix: Option<String>,
    /// Config file to use
    #[clap(short, long, value_parser)]
    config: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    //if a config file was passed as an argument, use that. Otherwise, fall back to the default config file.
    let config_file = match args.config.as_deref() {
        Some(x) => x,
        None => "config.json",
    };

    let mut config = Config::new(config_file);

    //if any config options were passed on the command line, use those values.
    if let Some(t) = args.token.as_deref() {
        config.set_token(t);
    }
    if let Some(p) = args.prefix.as_deref() {
        config.set_prefix(p);
    }

    let token: String = config.get_token();

    let framework = StandardFramework::new()
        .configure(|c| c.case_insensitivity(true).prefix(config.get_prefix()))
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
