use std::{sync::Arc, time::Duration, collections::HashMap, vec};

use serenity::{
    builder::CreateApplicationCommand,
    collector::{CollectComponentInteraction, CollectModalInteraction},
    model::prelude::{
        application::interaction::Interaction,
        command::CommandOptionType,
        component::{ActionRowComponent, ButtonStyle, InputTextStyle},
        interaction::{
            message_component::{MessageComponentInteraction, MessageComponentInteractionData},
            InteractionResponseType, InteractionType, modal::ModalSubmitInteraction,
        },
    },
    prelude::*, futures::stream::ForEach,
};

//use crate::cp2020::skill;

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
    let (role, role_response) = ask_role(&interaction.clone(), ctx).await;
    let (mut skill_list, skill_list_response) = ask_skills(role_response.clone(), ctx).await;
    //let (ma, ex, ot) = (skill_list.contains(&String::from("Martial Art")), skill_list.contains(&String::from("Expert")), skill_list.contains(&String::from("Other")));
    let (ma, ex, ot) = (skill_list.iter().any(|i| i.eq_ignore_ascii_case(&String::from("Martial Art"))),  skill_list.iter().any(|i| i.eq_ignore_ascii_case(&String::from("Expert"))),  skill_list.iter().any(|i| i.eq_ignore_ascii_case(&String::from("Other"))));
    if ma || ex || ot {
        let mut more_skills = ask_more_skills(skill_list_response.clone(), ctx, ma, ex, ot).await;
        skill_list.append(&mut more_skills);
    }


    println!("USER ENTERED:\nRole: {}\nSkills: {:?}", role, skill_list);

}

async fn ask_role(
    interaction: &Interaction,
    ctx: &Context,
) -> (String, Arc<MessageComponentInteraction>) {
    let _message = interaction
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
                                row2.create_button(|submit_role| {
                                    submit_role
                                        .custom_id("SubRoleBtn")
                                        .label("Submit")
                                        .style(ButtonStyle::Primary)
                                })
                            })
                        }).ephemeral(true)
                })
        })
        .await;

    let mut response_data: Vec<String> = vec![];

    loop {
        let response = CollectComponentInteraction::new(&ctx.shard)
            .author_id(
                interaction
                    .to_owned()
                    .application_command()
                    .unwrap()
                    .user
                    .id,
            )
            .timeout(Duration::from_secs(600))
            .await;

        if response.is_none() {
            println!("Error processing response");
        }
        let response = response.unwrap();

        if response.data.custom_id == "SubRoleBtn" && response_data.len() > 0 {
            println!("Submit: {:?}", response_data[0].clone());
            _ = response.delete_original_interaction_response(ctx).await;
            return (response_data[0].to_string(), response);
        } else if response.data.custom_id != "SubRoleBtn" {
            response_data = response.data.values.clone();
            println!("Data: {:?}", response_data);
        }

        if let Err(why) = response
            .create_interaction_response(&ctx.http, |rsp| {
                rsp.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| msg.content(format!("{:?}", response_data)).ephemeral(true))
            })
            .await
        {
            println!("Error sending response: {:?}", why);
        }
    }
}

async fn ask_skills(
    interaction: Arc<MessageComponentInteraction>,
    ctx: &Context,
) -> (Vec<String>, Arc<MessageComponentInteraction>) {
    let _message = interaction
        .to_owned()
        .create_interaction_response(&ctx.http, |rsp| {
            rsp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|response| {
                    response
                        .custom_id("InputChar")
                        .title("Input Character Details")
                        .components(|rows| {
                            rows.create_action_row(|row2| {
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
                                            options.create_option(|opt| {
                                                opt.value("Other").label("Other")
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
                                                opt.value("Awareness/Notice")
                                                    .label("Awareness/Notice")
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
                                                opt.value("Diagnose Illness")
                                                    .label("Diagnose Illness")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Education & Gen Knowledge")
                                                    .label("Education & Gen Knowledge")
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
                                                opt.value("System Knowledge")
                                                    .label("System Knowledge")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Teaching").label("Teaching")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Wilderness Survival")
                                                    .label("Wilderness Survival")
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
                                                opt.value("Operate Hvy Machinery")
                                                    .label("Operate Hvy Machinery")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pilot (Gyro)").label("Pilot (Gyro)")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pilot (Fixed Wing)")
                                                    .label("Pilot (Fixed Wing)")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pilot (Dirigible)")
                                                    .label("Pilot (Dirigible)")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pilot (Vect Thrust)")
                                                    .label("Pilot (Vect Thrust)")
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
                            .create_action_row(|row5| {
                                row5.create_select_menu(|skills4_menu| {
                                    skills4_menu
                                        .custom_id("SkillsMenu4")
                                        .placeholder("TECH skills")
                                        .min_values(0)
                                        .max_values(20)
                                        .options(|options| {
                                            options.create_option(|opt| {
                                                opt.value("Aero Tech").label("Aero Tech")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("AV Tech").label("AV Tech")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Basic Tech").label("Basic Tech")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Cryotank Operation")
                                                    .label("Cryotank Operation")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Cyberdeck Design")
                                                    .label("Cyberdeck Design")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Cyber Tech").label("Cyber Tech")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Demolitions").label("Demolitions")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Disguise").label("Disguise")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Electronics").label("Electronics")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Elec Security").label("Elec Security")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("First Aid").label("First Aid")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Forgery").label("Forgery")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Gyro Tech").label("Gyro Tech")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Paint or Draw").label("Paint or Draw")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Photo & Film").label("Photo & Film")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pharmaceuticals")
                                                    .label("Pharmaceuticals")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pick Lock").label("Pick Lock")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Pick Pocket").label("Pick Pocket")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Play Instrument")
                                                    .label("Play Instrument")
                                            });
                                            options.create_option(|opt| {
                                                opt.value("Weaponsmith").label("Weaponsmith")
                                            });
                                            options
                                        })
                                })
                            })
                            .create_action_row(|row6| {
                                row6.create_button(|submit_skills| {
                                    submit_skills
                                        .custom_id("SubSkillsBtn")
                                        .label("Submit")
                                        .style(ButtonStyle::Primary)
                                })
                            })
                        }).ephemeral(true)
                })
        })
        .await;

    let mut response_data: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let response = CollectComponentInteraction::new(&ctx.shard)
            .author_id(
                interaction
                    .to_owned()
                    .user
                    .id,
            )
            .timeout(Duration::from_secs(600))
            .await;

        if response.is_none() {
            println!("Error processing response");
        }
        let response = response.unwrap();
        

        if response.data.custom_id == "SubSkillsBtn" {
            println!("Submit: {:?}", response_data.clone());
            let mut skill_list: Vec<String> = vec![];
            for k in response_data.keys() {
                skill_list.append(response_data[k].clone().as_mut());
            }
            println!("skills: {:?}", skill_list);
            return (skill_list, response);
        } else if response.data.custom_id != "SubSkillsBtn" {
            println!("Data: {:?}", response.data.values.clone());
            response_data.insert(response.data.custom_id.to_string(), response.data.values.clone());
        }
        if let Err(why) = response
            .create_interaction_response(&ctx.http, |rsp| {
                rsp.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| msg.content(format!("{:?}", response_data)).ephemeral(true))
            })
            .await
        {
            println!("Error sending response: {:?}", why);
        }
    }

}

async fn ask_more_skills(interaction: Arc<MessageComponentInteraction>, ctx: &Context, m_art: bool, exp: bool, other: bool) -> Vec<String>
{
    let _message = interaction
    .to_owned()
    .create_interaction_response(&ctx.http, |rsp| {
        rsp.kind(InteractionResponseType::Modal)
        .interaction_response_data(|response| {
            response
                .custom_id("MoreSkillsInput")
                .title("Some skills need further specification. Please enter details below.")
                .components(|rows| {
                    if m_art {
                        rows.create_action_row(|mrow| {
                            mrow.create_input_text(|m_art_input| {
                                m_art_input
                                    .custom_id("MartialArts")
                                    .style(InputTextStyle::Paragraph)
                                    .label("Martial Arts, enter one per line")
                                    .required(true)  
                            })
                        });
                    }
                    if exp {
                        rows.create_action_row(|erow| {
                            erow.create_input_text(|expert_input| {
                                expert_input
                                    .custom_id("Expert")
                                    .style(InputTextStyle::Paragraph)
                                    .label("Expert, enter one per line")
                                    .required(true)  
                            })
                        });
                    }
                    if other {
                        rows.create_action_row(|orow| {
                            orow.create_input_text(|other_input| {
                                other_input
                                    .custom_id("Other")
                                    .style(InputTextStyle::Paragraph)
                                    .label("Other skills, enter as skill:attribute, one per line")
                                    .required(true)  
                            })
                        });
                    }
                    rows
               }).ephemeral(true)
        })
    }).await;

    let response = CollectModalInteraction::new(&ctx.shard)
        .author_id(
            interaction
                .to_owned()
                .user
                .id,
        )
        .timeout(Duration::from_secs(3600))
        .await;

    let rsp = response.clone().unwrap();

    let collected = rsp
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
                    "-".to_string()
                } else {
                    inp.to_owned().value
                }
            }
            ActionRowComponent::SelectMenu(inp) => inp.to_owned().values[0].to_string(),
            _ => format!("No match!"),
        })
        .collect::<Vec<String>>();

        return data;

}