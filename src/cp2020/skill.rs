use serenity::{
    builder::{CreateApplicationCommand},
    model::{
        prelude::{
            application::interaction::{Interaction}
        },
    },
    prelude::*,
};

use crate::cp2020::common::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("skill")
        .description("Roll a skill")
        .create_option(|option| {
            option
                .name("skill")
                .description("Which skill to roll")
                .required(true)  
        })
}

pub async fn run(interaction: Interaction, ctx: &Context) {
    if let Some(app) = interaction.application_command() {
        let serverid = format!("{}", app.guild_id.unwrap_or_default().0);
        let userid = format!("{}", app.user.id.0);
        let mut modifier: u32 = 0;
    let mut response: String = String::from("");
    if let Some(cid) = get_active_character(&serverid, &userid).await {
        if let Some(character) = get_character(cid).await {
            if let Some(skill) = &app.data.options.get(0).unwrap().value {
                let c: Vec<&Cp2020Skill> = character
                    .skills
                    .iter()
                    .filter(|&x| x.skill_name.to_lowercase() == skill.to_string().to_lowercase())
                    .collect();

                if c.len() > 0 {
                    modifier += (c[0].skill_value
                        + match c[0].skill_attribute.to_lowercase().as_str() {
                            "inteligence" => character.inteligence,
                            "reflex" => character.reflex,
                            "tech" => character.tech,
                            "cool" => character.cool,
                            "attractiveness" => character.attractiveness,
                            "movement" => character.movement,
                            "body" => character.body,
                            "empathy" => character.empathy,
                            _ => 0,
                        }) as u32
                }
                if character.role.to_lowercase() == "solo"
                    && skill.to_string().to_lowercase() == "awareness/notice"
                {
                    modifier += character.special_ability as u32;
                }
            }
        } else {
            response += "__***Active character could not be loaded!***__\n"
        }
    } else {
        response += "*No active character.*\n"
    };

    let result = d10_exploding();
    response = if result == 1 {
        format!("{}{} rolled a skill and critically failed!", response, app.user.name)
    } else {
        format!(
            "{}{} rolled a skill and got {}",
            response,
            app.user.name,
            result + modifier
        )
    };

    if let Err(why) = app.channel_id.say(&ctx.http, response).await {
        println!("Error sending message: {:?}", why);

    }
}
}

