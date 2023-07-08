use std::{sync::Arc, time::Duration, collections::HashMap, vec};

use serenity::{
    builder::{CreateApplicationCommand, },
    collector::{CollectComponentInteraction, CollectModalInteraction},
    model::prelude::{
        application::interaction::Interaction,
        command::CommandOptionType,
        component::{ActionRowComponent, ButtonStyle, InputTextStyle},
        interaction::{
            message_component::{MessageComponentInteraction,},
            InteractionResponseType, modal::ModalSubmitInteraction,
        },
    },
    prelude::*, 
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
    let (role, role_response) = ask_role(&interaction.clone(), ctx).await;

    let (_stats, _humanity, stats_response) = ask_stats(role_response.clone(), ctx).await;

    let (mut skill_list, flags, skill_list_response) = ask_skills(stats_response.clone(), ctx).await;

    if flags > 0 {
        let mut more_skills = ask_more_skills(skill_list_response.clone(), ctx, flags).await;
        skill_list.append(&mut more_skills);
        println!("{more_skills:?}\n");
    }
    // TODO:
    // Get values for special ability, and for each skill in skill_list from user.
    // build Cp2020Character from role, stats, and skill_list.
    // save built character.

    println!("USER ENTERED:\nRole: {role}\nSkills: {skill_list:?}");
}

async fn ask_role(
    interaction: &Interaction,
    ctx: &Context,
) -> (String, Arc<MessageComponentInteraction>) {
    let roles = ["Rocker","Solo","Netrunner","Techie","MedTechie","Media","Cop","Corporate","Fixer","Nomad"];
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
                                            roles.into_iter().for_each(|s| {
                                                options.create_option(|o| {
                                                    o.value(s).label(s)
                                                });
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

async fn ask_stats(interaction: Arc<MessageComponentInteraction>, ctx: &Context) -> ( Vec<i8>, f32, Arc<ModalSubmitInteraction> ) {
    //println!("Launched ask_stats function");
    let message = interaction
        .create_interaction_response(&ctx.http, |rsp| {
            rsp.kind(InteractionResponseType::Modal)
            .interaction_response_data(|response| {
                response
                .custom_id("stats_input")
                .title("Input Stats")
                .components(|rows| {
                    rows.create_action_row(|srow| {
                        srow.create_input_text(|stats_input| {
                            stats_input
                            .custom_id("StatsLine")
                            .style(InputTextStyle::Short)
                            .label("Input\nINT REF TECH COOL ATTR LUCK MA BODY EMP")
                            .required(true)
                        })
                    }).create_action_row(|irow| {
                        irow.create_input_text(|stats_input| {
                            stats_input
                            .custom_id("InitBonus")
                            .style(InputTextStyle::Short)
                            .label("Enter your initiative modifier")
                            .required(true)
                        })
                    }).create_action_row(|hrow| {
                        hrow.create_input_text(|stats_input| {
                            stats_input
                            .custom_id("HumanityRow")
                            .style(InputTextStyle::Short)
                            .label("Enter your Humanity score")
                            .required(true)
                        })
                    })
                })
            })
        }).await;
    
    if message.is_err() {println!("results of creating modal: {message:?}");}
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

    let mut stats: Vec<i8> = vec![];
    let mut init: i8 = 0;
    let mut humanity: f32 = 0.0;

    collected.to_owned().iter().for_each(|x| {
        match x {
            ActionRowComponent::InputText(inp) => {
                match inp.custom_id.as_str() {
                    "StatsLine" => {
                        for l in inp.value.lines() {
                            println!("xx{l}");
                            if l != "" {
                                let x: Vec<&str> = l.split(' ').into_iter().collect();
                                if x.len() == 10 {
                                    for i in x {
                                        let stat = match i.parse::<i8>() {
                                            Ok(n) => {n}
                                            Err(x) => {
                                                println!("{:?}\n{:?} is not a valid integer!", x, i);
                                                -127
                                            }
                                        };
                                        stats.push(stat);
                                    }
                                }
                                else {
                                    println!("l: {:?}, x: {:?}, len: {} is not a valid stats line!",l,x,x.len());
                                }
                            }
                        }
                    }
                    "InitBonus" => {
                        for l in inp.value.lines() {
                            if l != "" {
                                init = match l.parse::<i8>() {
                                    Ok(n) => {n}
                                    Err(x) => {
                                        println!("{:?}\n{:?} is not a valid integer!", x, l);
                                        -127
                                    }
                                };

                            }
                        }
                    }
                    "HumanityRow" => {
                        for l in inp.value.lines() {
                            if l != "" {
                                humanity = match l.parse::<f32>() {
                                    Ok(n) => {n}
                                    Err(x) => {
                                        println!("{:?}\n{:?} is not a valid value for humanity!", x, l);
                                        0.0 
                                    }
                                }
                            }
                        }
                    }
                    _ => {println!("Error! {} is not an expected stat input!", inp.custom_id);}
                }
            }
            _ => {println!("{:?}",x)}
        }
    });
    stats.push(init);
    (stats, humanity, rsp)

}

async fn ask_skills(
    interaction: Arc<ModalSubmitInteraction>,
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
            for v in response_data.values() {
                for s in v {
                    if s == "Martial Art" {flags = flags | MA}
                    else if s == "Expert" {flags = flags | EX}
                    else if s == "Language" {flags = flags | LA}
                    else if s == "Other" {flags = flags | OT}
                    else if attrskills.contains(&s.as_str()) {skill_list.push(Cp2020Skill { skill_name: s.clone(), skill_attribute: String::from("ATTR"), skill_value: 0 })}
                    else if bodyskills.contains(&s.as_str()) {skill_list.push(Cp2020Skill { skill_name: s.clone(), skill_attribute: String::from("BODY"), skill_value: 0 })}
                    else if coolskills.contains(&s.as_str()) {skill_list.push(Cp2020Skill { skill_name: s.clone(), skill_attribute: String::from("COOL/WILL"), skill_value: 0 })}
                    else if empskills.contains(&s.as_str()) {skill_list.push(Cp2020Skill { skill_name: s.clone(), skill_attribute: String::from("EMP"), skill_value: 0 })}
                    else if intskills.contains(&s.as_str()) {skill_list.push(Cp2020Skill { skill_name: s.clone(), skill_attribute: String::from("INT"), skill_value: 0 })}
                    else if refskills.contains(&s.as_str()) {skill_list.push(Cp2020Skill { skill_name: s.clone(), skill_attribute: String::from("REF"), skill_value: 0 })}
                    else if techskills.contains(&s.as_str()) {skill_list.push(Cp2020Skill { skill_name: s.clone(), skill_attribute: String::from("TECH"), skill_value: 0 })}
                    else {println!("{:?} is not ta valid skill!",s)}  
                } 
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
//    .to_owned()
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

// TODO: Add function to get skill values.
//async fn get_skill_values()