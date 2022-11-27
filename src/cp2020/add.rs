use std::time::Duration;

use serenity::{
    builder::CreateApplicationCommand,
    collector::{CollectComponentInteraction, CollectModalInteraction},
    model::prelude::{
        application::interaction::Interaction,
        command::CommandOptionType,
        component::{ActionRowComponent, InputTextStyle},
        interaction::InteractionResponseType,
    },
    prelude::*,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add")
        .description("Add a character")
        .create_option(|option| {
            option
                .name("Name")
                .description("The name of the character to add.")
                .required(true)
                .kind(CommandOptionType::String)
        })
}

pub async fn run(interaction: &Interaction, ctx: &Context) {
    let message = interaction
        .to_owned()
        .application_command()
        .unwrap()
        .create_interaction_response(&ctx.http, |rsp| {
            rsp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|response| {
                    response
                        .custom_id("InputChar")
                        .title("Input Character Details")
                        .components(|rows| {
                            rows.create_action_row(|row1| {
                                row1.create_select_menu(|role_menu| {
                                    role_menu
                                        .custom_id("role")
                                        .placeholder("Role")
                                        .max_values(1)
                                        .min_values(1)
                                        .options(|options| {
                                            options.create_option(|opt| {
                                                opt.value("Rocker").label("Rocker")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Solo").label("Solo")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Netrunner").label("Netrunner")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Techie").label("Techie")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Medtechie").label("Medtechie")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Media").label("Media")
                                            });
                                            options
                                                .create_option(|opt| opt.value("Cop").label("Cop"));
                                            options.create_option(|opt| {
                                                opt.value("Corporate").label("Corporate")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Fixer").label("Fixer")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Nomad").label("Nomad")
                                            });
                                            options
                                        })
                                })
                            })
                            .create_action_row(|row2| {
                                row2.create_select_menu(|skills1_menu| {
                                    skills1_menu
                                        .custom_id("SkillsMenu1")
                                        .placeholder("ATTR, BODY, & COOL/WILL skills")
                                        .min_values(0)
                                        .max_values(10)
                                        .options(|options| {
                                            options.create_option(|opt| {
                                                opt.value("Personal Grooming")
                                                    .label("Personal Grooming")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Wardrobe & Style")
                                                    .label("Wardrobe & Style")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Endurance").label("Endurance")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Strength Feat").label("Strength Feat")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Swimming").label("Swimming")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Interrogation").label("Interrogation")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Intimidate").label("Intimidate")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Oratory").label("Oratory")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Resist Torture/Drugs")
                                                    .label("Resist Torture/Drugs")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Streetwise").label("Streetwise")
                                            });
                                            options
                                        })
                                })
                            })
                        })
                })

            // rsp.kind(InteractionResponseType::Modal)
            //     .interaction_response_data(|response| {
            //         response
            //             .custom_id("InputChar")
            //             .title("Input Character Details")
            //             .components(|rows| {
            //                 rows.create_action_row(|row1| {
            //                     row1.create_input_text(|name_input| {
            //                         name_input
            //                             .custom_id("CharName")
            //                             .style(InputTextStyle::Short)
            //                             .label("Name")
            //                             .required(true)
            //                     })
            //                 })
        })
        .await;

    if let Err(why) = message {
        println!("Error displaying modal: {:?}", why);
        return;
    }

    // let response1 = CollectComponentInteraction::new(&ctx.shard)
    //     .author_id(
    //         interaction
    //             .to_owned()
    //             .application_command()
    //             .unwrap()
    //             .user
    //             .id
    //     )
    //     .timeout(Duration::from_secs(3600))
    //     .await;

    // if response1.is_none() {
    //     println!("Error processing response");
    //     return;
    // }
    // let response1 = response1.unwrap();

    // let response1_data = response1
    //     .data
    //     .values
    //     .clone();

    let response = CollectModalInteraction::new(&ctx.shard)
        .author_id(
            interaction
                .to_owned()
                .application_command()
                .unwrap()
                .user
                .id,
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
            ActionRowComponent::SelectMenu(inp) => inp.to_owned().values[0].to_string(),
            _ => format!("No match!"),
        })
        .collect::<Vec<String>>();

    if let Err(why) = response
        .create_interaction_response(&ctx.http, |rsp| {
            rsp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| {
                    msg.content(format!("{:?}", data))
                })
        })
        .await
    {
        println!("Error sending response: {:?}", why);
    }
}
