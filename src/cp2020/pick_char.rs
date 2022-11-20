use std::sync::Arc;

use serenity::{
    builder::CreateApplicationCommand, model::prelude::{application::interaction::Interaction, command::CommandOptionType},
    prelude::*,
};

use crate::cp2020::common::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("pick_char")
        .description("Add a character")
        .create_option(|option| {
            option
                .name("character")
                .description("name or number of character to select")
                .required(false)
                .kind(CommandOptionType::String)
        })
}

pub async fn run(interaction: Interaction, ctx: &Context) {
    if let Some(app) = interaction.application_command() {
        let serverid = format!("{}", app.guild_id.unwrap_or_default().0);
        let userid = format!("{}", app.user.id.0);
        let characters = get_character_list(&serverid, &userid).await;
        let mut response = String::from("");
        if characters.len() == 0 {
            response += "You do not have any characters saved on this server!";
        } else if app.data.options.is_empty() {
            for i in 0..characters.len() {
                response += format!(
                    "{}: {} ({})\n",
                    i + 1,
                    characters[i].character_name.as_str(),
                    characters[i].role.as_str()
                )
                .as_str();
            }
        } else {
            if let Some(a) = &app.data.options.get(0).unwrap().value {
                if let Ok(chr_num) = a.to_string().parse::<u32>() {
                    let c = &characters[(chr_num - 1) as usize];
                    if !set_active_character(&serverid, &userid, c.id).await {
                        response = String::from("Error setting character!");
                    } else {
                        response +=
                            format!("Set active character to {} ({})", c.character_name, c.role)
                                .as_str()
                    }
                } else {
                    let chars: Vec<&Cp2020Character> = characters
                        .iter()
                        .filter(|&c| c.character_name.to_lowercase() == a.to_string().to_lowercase())
                        .collect();
                    if chars.len() > 0 {
                        if !set_active_character(&serverid, &userid, chars[0].id).await {
                            response = String::from("Error setting character!");
                        } else {
                            response += format!(
                                "Set active character to {} ({})",
                                chars[0].character_name, chars[0].role
                            )
                            .as_str();
                        }
                    } else {
                        response += format!("Unable to find saved character {}", a).as_str();
                    }
                }
            }
        }
        if let Err(why) = app.channel_id.say(&ctx.http, response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
