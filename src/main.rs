#[macro_use]
extern crate lazy_static;

mod config;
mod cp2020_functions;
mod dbinterface;
mod dice_commands;
mod cp2020;

use crate::config::config::Config;
use crate::dice_commands::dice_commands::GENERAL_GROUP;

use clap::Parser;
use cp2020::common::cp2020_init;
use lazy_static::lazy_static;
use serenity::{
    async_trait, framework::standard::StandardFramework, model::{gateway::Ready, id::GuildId, prelude::interaction::{Interaction}}, prelude::*, 
};
use std::sync::RwLock;

lazy_static! {
    static ref CONF: RwLock<Config> = RwLock::new(Config::new());
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction.clone() {
             match command.data.name.as_str() {
                  "add" => cp2020::add::run(&interaction, &ctx).await,
                  "init" => cp2020::init::run(interaction, &ctx).await,
                  "pick_char" => cp2020::pick_char::run(interaction, &ctx).await,
                  "skill" => cp2020::skill::run(interaction, &ctx).await,
                 _ => (),
             };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let ids = CONF.read().as_deref().unwrap().get_command_guilds();
        for id in ids {
        let guild_id = GuildId(id.parse().expect("error parsing GuildID"));
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| cp2020::add::register(command))
                .create_application_command(|command| cp2020::init::register(command))
                .create_application_command(|command| cp2020::pick_char::register(command))
                .create_application_command(|command| cp2020::skill::register(command))
        })
        .await;
    
    
        println!("I now have the following guild slash commands: {:#?} on guild {:?}", commands, guild_id);
    }

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

    dbinterface::init_db().await;

    cp2020_init().await;

    let framework = StandardFramework::new()
        .configure(|c| {
            c.case_insensitivity(true)
                .prefix(CONF.read().as_deref().unwrap().get_prefix())
        })
        .group(&GENERAL_GROUP)
        .group(&cp2020_functions::CPCOMMANDS_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Couldn't create the new client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }


    //cp2020_functions::cp2020_init().await;
}
