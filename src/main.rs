#[macro_use]
extern crate lazy_static;

mod config;
mod dbinterface;
mod dice_commands;
mod cp2020_functions;

use crate::config::config::Config;
use crate::dice_commands::dice_commands::GENERAL_GROUP;
use crate::cp2020_functions::cp2020_functions::CP_COMMANDS_GROUP;

use clap::Parser;
use lazy_static::lazy_static;
use serenity::{
    async_trait, framework::standard::StandardFramework, model::gateway::Ready, prelude::*,
};
use std::sync::RwLock;

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
    #[clap(short, long, value_parser)]
    db_url: Option<String>,
}

lazy_static! {
    static ref CONF: RwLock<Config> = RwLock::new(Config::new());
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    //if a config file was passed as an argument, use that. Otherwise, fall back to the default config file.
    let config_file = match args.config.as_deref() {
        Some(x) => x,
        None => "config.json",
    };

    loop {
        if let Ok(c) = CONF.try_write().as_deref_mut() {
            c.parse_config(config_file);
            break;
        }
    }

    //if any config options were passed on the command line, use those values.
    if let Some(t) = args.token.as_deref() {
        loop {
            if let Ok(c) = CONF.try_write().as_deref_mut() {
                c.set_token(t);
                break;
            }
        }
    };
    if let Some(p) = args.prefix.as_deref() {
        loop {
            if let Ok(c) = CONF.try_write().as_deref_mut() {
                c.set_prefix(p);
                break;
            }
        }
    }

    if let Some(db) = args.db_url.as_deref() {
        loop {
            if let Ok(c) = CONF.try_write().as_deref_mut() {
                c.set_db_url(db);
                break;
            }
        }
    }
    let token: String = CONF.read().as_deref().unwrap().get_token();

    let framework = StandardFramework::new()
    .configure(|c| {
        c.case_insensitivity(true)
            .prefix(CONF.read().as_deref().unwrap().get_prefix())
    })
    .group(&GENERAL_GROUP)
    .group(&CP_COMMANDS_GROUP);

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
