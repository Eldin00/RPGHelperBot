use std::{sync::Arc, time::Duration, collections::HashMap, vec};

use serenity::{
    builder::{CreateApplicationCommand, CreateSelectMenu},
    collector::{CollectComponentInteraction, CollectModalInteraction},
    model::prelude::{
        application::interaction::Interaction,
        command::CommandOptionType,
        component::{ActionRowComponent, ButtonStyle, InputTextStyle},
        interaction::{
            message_component::{MessageComponentInteraction, /*MessageComponentInteractionData*/},
            InteractionResponseType, /*InteractionType, modal::ModalSubmitInteraction,*/
        },
    },
    prelude::*, /*futures::stream::ForEach,*/
};

use super::common::Cp2020Skill;

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
    let mut skill_list: Vec<Cp2020Skill> = vec![];
    let (role, role_response) = ask_role(&interaction.clone(), ctx).await;
    let (mut skill_list, flags, skill_list_response) = ask_skills(role_response.clone(), ctx).await;
    if flags > 0 {
        let more_skills = ask_more_skills(skill_list_response.clone(), ctx, flags).await;
        println!("{more_skills:?}\n");
        //let tmp_skill: Vec<Cp2020Skill> = more_skills.into_iter().
        // skill_list.append(&mut more_skills);
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
) -> (Vec<Cp2020Skill>, i8, Arc<MessageComponentInteraction>) {
    let attrskills = ["Personal Grooming", "Wardrobe & Style"];
    let bodyskills = ["Endurance", "Strength Feat", "Swimming"];
    let coolskills = ["Interrogation", "Intimidate", "Oratory", "Resist Torture/Drugs", "Streetwise"];
    let empskills = ["Human Perception", "Interview", "Leadership", "Seduction", "Social", "Persuasion & Fast Talk", "Perform"];
    let intskills = ["Accounting", "Anthropology", "Awareness/Notice", "Biology", "Botany","Chemistry","Composition","Diagnose Illness", "Education & Gen. Know.", "Expert", "Gamble", "Geology", "Hide/Evade", "History", "Language", "Library Search", "Mathematics", "Physics", "Programming", "Shadow/Track", "Stock Market", "System Knowledge", "Teaching", "Wilderness Survival", "Zoology"];
    let refskills = ["Archery", "Athletics", "Brawling", "Dance", "Dodge & Escape", "Driving", "Fencing", "Handgun", "Heavy Weapons", "Martial Art", "Melee", "Motorcycle", "Operate Hvy. Machinery", "Pilot(Gyro)", "Pilot(Fixed Wing)", "Pilot(Dirigible)", "Pilot(Vect. Thrust)", "Rifle", "Stealth", "Submachinegun"];
    let techskills = ["Aero Tech", "AV Tech", "Basic Tech", "Cryotank Operation", "Cyberdeck Design", "CyberTech", "Demolitions", "Disguise", "Electronics", "Elect. Security", "First Aid", "Forgery", "Gyro Tech", "Paint or Draw", "Photo & Film", "Pharmacuticals", "Pick Lock", "Pick Pocket", "Play Instrument", "Weaponsmith"];

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
                                        .placeholder("ATTR, BODY, COOL/WILL & EMP skills")
                                        .min_values(0)
                                        .max_values(17)
                                        .options(|options| {
                                            attrskills.into_iter().for_each(|s| {
                                                options.create_option(|o| {
                                                    o.value(s).label(s)
                                                });
                                            });
                                            bodyskills.into_iter().for_each(|s| {
                                                options.create_option(|o| {
                                                    o.value(s).label(s)
                                                });
                                            });
                                            coolskills.into_iter().for_each(|s| {
                                                options.create_option(|o| {
                                                    o.value(s).label(s)
                                                });
                                            });
                                            empskills.into_iter().for_each(|s| {
                                                options.create_option(|o| {
                                                    o.value(s).label(s)
                                                });
                                            });
                                            options.create_option(|o| {o.value("Other").label("Other")});
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
                                            intskills.into_iter().for_each(|s| {
                                                options.create_option(|o| { o.value(s).label(s)});
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
                                            refskills.into_iter().for_each(|s| {
                                                options.create_option(|o| {o.value(s).label(s)});
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
                                            techskills.into_iter().for_each(|s| {
                                                options.create_option(|o| {o.value(s).label(s)});
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
        
        let mut flags :i8 = 0;
        const MA :i8 = 1;
        const EX :i8 = 2;
        const LA :i8 = 4;
        const OT :i8 = 8;
        if response.data.custom_id == "SubSkillsBtn" {
            println!("Submit: {:?}", response_data.clone());
            let mut skill_list: Vec<Cp2020Skill> = vec![];
            for k in response_data.keys() {
                println!("{k}\n");
                if k == "Martial Art" {flags = flags | MA}
                else if k == "Expert" {flags = flags | EX}
                else if k == "Language" {flags = flags | LA}
                else if k == "Other" {flags = flags | OT}
                else if attrskills.contains(&k.as_str()) {skill_list.push(Cp2020Skill { skill_name: k.clone(), skill_attribute: String::from("ATTR"), skill_value: 0 })}
                else if bodyskills.contains(&k.as_str()) {skill_list.push(Cp2020Skill { skill_name: k.clone(), skill_attribute: String::from("BODY"), skill_value: 0 })}
                else if coolskills.contains(&k.as_str()) {skill_list.push(Cp2020Skill { skill_name: k.clone(), skill_attribute: String::from("COOL/WILL"), skill_value: 0 })}
                else if empskills.contains(&k.as_str()) {skill_list.push(Cp2020Skill { skill_name: k.clone(), skill_attribute: String::from("EMP"), skill_value: 0 })}
                else if intskills.contains(&k.as_str()) {skill_list.push(Cp2020Skill { skill_name: k.clone(), skill_attribute: String::from("INT"), skill_value: 0 })}
                else if refskills.contains(&k.as_str()) {skill_list.push(Cp2020Skill { skill_name: k.clone(), skill_attribute: String::from("REF"), skill_value: 0 })}
                else if techskills.contains(&k.as_str()) {skill_list.push(Cp2020Skill { skill_name: k.clone(), skill_attribute: String::from("TECH"), skill_value: 0 })}
                else {println!("{:?} is not ta valid skill!",k)}   
            }
            println!("skills: {:?}", skill_list);
            return (skill_list, flags, response);
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

async fn ask_more_skills(interaction: Arc<MessageComponentInteraction>, ctx: &Context, flags: i8) -> Vec<Cp2020Skill>
{
    const MA :i8 = 1;
    const EX :i8 = 2;
    const LA :i8 = 4;
    const OT :i8 = 8;

    let _message = interaction
    .to_owned()
    .create_interaction_response(&ctx.http, |rsp| {
        rsp.kind(InteractionResponseType::Modal)
        .interaction_response_data(|response| {
            response
                .custom_id("MoreSkillsInput")
                .title("Some skills need further specification. Please enter details below.")
                .components(|rows| {
                    if (flags & MA) > 0 {
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
                    if (flags & EX) > 0 {
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
                    if (flags & LA) > 0 {
                        rows.create_action_row(|erow| {
                            erow.create_input_text(|language_input| {
                                language_input
                                    .custom_id("Language")
                                    .style(InputTextStyle::Paragraph)
                                    .label("Language, enter one per line")
                                    .required(true)  
                            })
                        });
                    }
                    if (flags & OT) > 0 {
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

    let mut skills: Vec<Cp2020Skill> = vec![];
    collected.to_owned().iter().for_each(|x| {
        match x {
            ActionRowComponent::InputText(inp) => {
                match inp.custom_id.as_str() {
                    "MartialArts" => {
                        for l in inp.value.lines() {
                            if l != "" {skills.push(Cp2020Skill { skill_name: format!("Martial Art: {}",l), skill_attribute: String::from("REF"), skill_value: 0 });}
                        }
                    }
                    "Expert" => {
                        for l in inp.value.lines() {
                            if l != "" {skills.push(Cp2020Skill { skill_name: format!("Expert: {}",l), skill_attribute: String::from("INT"), skill_value: 0 });}
                        }
                    }
                    "Language" => {
                        for l in inp.value.lines() {
                            if l != "" {skills.push(Cp2020Skill { skill_name: format!("Language: {}",l), skill_attribute: String::from("INT"), skill_value: 0 });}
                        }
                    }
                    "Other" => {
                        for l in inp.value.lines() {
                            if l != "" {
                            let skill :Vec<&str> = l.split(':').into_iter().collect();
                                if skill.len() == 2 {
                                    skills.push(Cp2020Skill { skill_name: skill[0].to_string(), skill_attribute: skill[1].to_string(), skill_value: 0 })
                                }
                                else {println!("{l} was not a correctly formatted skill!")}
                            }
                        }
                    }
                    _ => {println!("Error! {} is not an expected skill type!", inp.custom_id);}
                }
            }
            _ => {println!("{:?}",x)}
        }
    });
    skills
}