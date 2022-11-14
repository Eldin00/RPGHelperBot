use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            application::interaction::{Interaction},
        },
    },
    prelude::*,
};

use crate::cp2020::common::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("init").description("Roll initiative")
}

pub async fn run(interaction: Interaction, ctx: &Context) {
    if let Some(app) = interaction.application_command() {
        let serverid = format!("{}", app.guild_id.unwrap_or_default().0);
        let userid = format!("{}", app.user.id.0);
        let mut modifier: u32 = 0;
        let mut response: String = String::from("");
        if let Some(cid) = get_active_character(&serverid, &userid).await {
            if let Some(character) = get_character(cid).await {
                modifier += character.initiative as u32;
                if character.role.to_lowercase() == "solo" {
                    modifier += character.special_ability as u32;
                }
            } else {
                response += "__***Active character could not be loaded!***__\n"
            }
        } else {
            response += "*No active character.*\n"
        };

        let result = d10_exploding();
        response = format!(
            "{}{} rolled for initiative and got:\n {}",
            response,
            app.user.name,
            result + modifier
        );

        if let Err(why) = app.channel_id.say(&ctx.http, response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
