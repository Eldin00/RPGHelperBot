use std::time::Duration;

use serenity::{
    builder::CreateApplicationCommand, model::prelude::{application::interaction::Interaction, interaction::InteractionResponseType, component::{InputTextStyle, ActionRowComponent}, },
    prelude::*, collector::CollectModalInteraction,
};

//use crate::cp2020::common::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("add").description("Add a character")
}

pub async fn run(interaction: &Interaction, ctx: &Context) {
    let modal = interaction
        .to_owned()
        .application_command()
        .unwrap()
        .create_interaction_response(&ctx.http, |rsp| {
            rsp.kind(InteractionResponseType::Modal)
                .interaction_response_data(|response| {
                    response
                        .custom_id("InputChar")
                        .title("Input Character Details")
                        .components(|rows| {
                            rows.create_action_row(|row1| {
                                row1.create_input_text(|name_input| {
                                    name_input
                                        .custom_id("CharName")
                                        .style(InputTextStyle::Short)
                                        .label("Name")
                                        .required(true)
                                })
                                // row1.create_select_menu(|roleMenu| {
                                //     roleMenu
                                //         .custom_id("Role")
                                //         .
                                // })
                            })
                        
                        })

                })
        }).await;

        if let Err(why) = modal {
            println!("Error displaying modal: {:?}", why);
            return;
        }

        let response = CollectModalInteraction::new(&ctx.shard)
        .author_id(
            interaction
                .to_owned()
                .application_command()
                .unwrap()
                .user
                .id
        )
        .timeout(Duration::from_secs(3600))
        .await;
        
        if response.is_none() {
            println!("Error processing response");
            return;
        }
        let response = response.unwrap();
        
        let collected = response
        .data
        .components
        .to_owned()
        .into_iter()
        .flat_map(|x| x.to_owned().components)
        .collect::<Vec<ActionRowComponent>>();

    let data = collected
        .to_owned()
        .iter()
        .map(|x| match x {
            ActionRowComponent::InputText(inp) => {
                if inp.to_owned().value == "" {
                    return "-".to_string();
                } else {
                    inp.to_owned().value
                }
            }
            _ => format!("No match!")
        })
        .collect::<Vec<String>>();

        if let Some(app) = &interaction.clone().application_command() {
            if let Err(why) = app.channel_id.say(&ctx.http, format!("{:?}", data)).await {
                println!("Error sending message: {:?}", why);
            }
        }

}
