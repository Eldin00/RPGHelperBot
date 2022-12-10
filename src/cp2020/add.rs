use std::time::Duration;

use serenity::{
    builder::CreateApplicationCommand,
    collector::{CollectComponentInteraction, CollectModalInteraction},
    model::prelude::{
        application::interaction::Interaction,
        command::CommandOptionType,
        component::{ActionRowComponent, InputTextStyle, ButtonStyle},
        interaction::{InteractionResponseType, InteractionType, message_component::MessageComponentInteractionData},
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
                            .create_action_row(|row3| {
                                row3.create_select_menu(|skills2_menu| {
                                    skills2_menu
                                        .custom_id("SkillsMenu2")
                                        .placeholder("INT skills")
                                        .min_values(0)
                                        .max_values(25)
                                        .options(|options| {
                                            options.create_option(|opt| {
                                                opt.value("Accounting").label("Accounting")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Anthropology").label("Anthropology")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Awareness/Notice").label("Awareness/Notice")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Biology").label("Biology")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Botany").label("Botany")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Chemistry").label("Chemistry")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Composition").label("Composition")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Diagnose Illness").label("Diagnose Illness")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Education & Gen Knowledge").label("Education & Gen Knowledge")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Expert").label("Expert")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Gamble").label("Gamble")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Geology").label("Geology")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Hide/Evade").label("Hide/Evade")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("History").label("History")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Language").label("Language")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Library Search").label("Library Search")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Mathematics").label("Mathematics")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Physics").label("Physics")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Programming").label("Programming")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Shadow/Track").label("Shadow/Track")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Stock Market").label("Stock Market")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("System Knowledge").label("System Knowledge")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Teaching").label("Teaching")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Wilderness Survival").label("Wilderness Survival")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Zoology").label("Zoology")
                                            });
                                            options
                                        })
                                    
                                })
                                
                            })
                            .create_action_row(|row4| {
                                row4.create_select_menu(|skills3_menu| {
                                    skills3_menu
                                        .custom_id("SkillsMenu3")
                                        .placeholder("REF skills")
                                        .min_values(0)
                                        .max_values(19)
                                        .options(|options| {
                                            options.create_option(|opt| {
                                                opt.value("Archery").label("Archery")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Athletics").label("Athletics")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Brawling").label("Brawling")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Dance").label("Dance")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Dodge & Escape").label("Dodge & Escape")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Fencing").label("Driving")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Handgun").label("Handgun")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Heavy Weapons").label("Heavy Weapons")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Martial Art").label("Martial Art")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Melee").label("Melee")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Motorcycle").label("Motorcycle")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Operate Hvy Machinery").label("Operate Hvy Machinery")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pilot (Gyro)").label("Pilot (Gyro)")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pilot (Fixed Wing)").label("Pilot (Fixed Wing)")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pilot (Dirigible)").label("Pilot (Dirigible)")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pilot (Vect Thrust)").label("Pilot (Vect Thrust)")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Rifle").label("Rifle")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Stealth").label("Stealth")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Submachinegun").label("Submachinegun")
                                            });
                                            options
                                        })
                                    })
                            })
                            // .create_action_row(|row5| {
                            //     row5.create_select_menu(|skills4_menu| {
                            //         skills4_menu
                            //             .custom_id("SkillsMenu4")
                            //             .placeholder("TECH skills")
                            //             .min_values(0)
                            //             .max_values(20)
                            //             .options(|options| {
                            //                 options.create_option(|opt| {
                            //                     opt.value("Aero Tech").label("Aero Tech")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("AV Tech").label("AV Tech")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Basic Tech").label("Basic Tech")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Cryotank Operation").label("Cryotank Operation")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Cyberdeck Design").label("Cyberdeck Design")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Cyber Tech").label("Cyber Tech")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Demolitions").label("Demolitions")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Disguise").label("Disguise")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Electronics").label("Electronics")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Elec Security").label("Elec Security")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("First Aid").label("First Aid")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Forgery").label("Forgery")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Gyro Tech").label("Gyro Tech")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Paint or Draw").label("Paint or Draw")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Photo & Film").label("Photo & Film")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Pharmaceuticals").label("Pharmaceuticals")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Pick Lock").label("Pick Lock")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Pick Pocket").label("Pick Pocket")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Play Instrument").label("Play Instrument")
                            //                 });
                            //                 options.create_option(|opt| {
                            //                     opt.value("Weaponsmith").label("Weaponsmith")
                            //                 });
                            //                 options
                            //             })
                            //     })
                            // })
                            .create_action_row(|row6| {
                                row6.create_button(|submit_skills| {
                                    submit_skills
                                        .custom_id("SubSkillsBtn")
                                        .label("Submit")
                                        .style(ButtonStyle::Primary)  
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
    
    let mut found: bool = false;
    while !found {
    let response1 = CollectComponentInteraction::new(&ctx.shard)
        .author_id(
            interaction
                .to_owned()
                //.message_component()
                .application_command()
                .unwrap()
                .user
                .id
        )
        .timeout(Duration::from_secs(600))
        .await;

    if response1.is_none() {
        println!("Error processing response");
        return;
    }
    let response1 = response1.unwrap();
    let mut response1_data: Vec<String> = vec![];

    if response1.data.custom_id == "SubSkillsBtn" {
        found = true;
    }
    else {
    response1_data = response1
        .data
        .values
        .clone();
    }
    // let response = CollectModalInteraction::new(&ctx.shard)
    //     .author_id(
    //         interaction
    //             .to_owned()
    //             .application_command()
    //             .unwrap()
    //             .user
    //             .id,
    //     )
    //     .timeout(Duration::from_secs(3600))
    //     .await;

    // if response.is_none() {
    //     println!("Error processing response");
    //     return;
    // }
    // let response = response.unwrap();

    // let collected = response
    //     .data
    //     .components
    //     .to_owned()
    //     .into_iter()
    //     .flat_map(|x| x.to_owned().components)
    //     .collect::<Vec<ActionRowComponent>>();

    // let data = collected
    //     .to_owned()
    //     .iter()
    //     .map(|x| match x {
    //         ActionRowComponent::InputText(inp) => {
    //             if inp.to_owned().value == "" {
    //                 return "-".to_string();
    //             } else {
    //                 inp.to_owned().value
    //             }
    //         }
    //         ActionRowComponent::SelectMenu(inp) => inp.to_owned().values[0].to_string(),
    //         _ => format!("No match!"),
    //     })
    //     .collect::<Vec<String>>();

    if let Err(why) = response1
        .create_interaction_response(&ctx.http, |rsp| {
            rsp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| {
                    msg.content(format!("{:?}", response1_data))
                })
        })
        .await
    {
        println!("Error sending response: {:?}", why);
    }
           }   
}
