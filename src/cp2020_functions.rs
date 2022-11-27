use rand::Rng;
use serenity::{
    client::Context,
    framework::standard::{macros::*, Args, CommandResult},
    futures::TryStreamExt,
    model::channel::Message,
};
use sqlx::Row;

use crate::dbinterface::DB_POOL;

struct Cp2020Skill {
    skill_name: String,
    skill_attribute: String,
    skill_value: u8,
}

struct Cp2020Character {
    id: i64,
    character_name: String,
    role: String,
    inteligence: u8,
    reflex: u8,
    initiative: u8,
    tech: u8,
    cool: u8,
    attractiveness: u8,
    movement: u8,
    body: u8,
    empathy: u8,
    special_ability: u8,
    skills: Vec<Cp2020Skill>,
}

#[group]
#[commands(cp_init, cp_skill, cp_add_char, cp_pick_char)]
pub struct CpCommands;

#[command]
async fn cp_init(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let serverid: String = format!("{}", msg.guild_id.unwrap().0);
    let userid: String = format!("{}", msg.author.id.0);
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
    let user = if msg.is_private() {
        "You"
    } else {
        msg.author.name.as_str()
    };
    let result = d10_exploding();
    response = format!(
        "{}{} rolled for initiative and got:\n {}",
        response,
        user,
        result + modifier
    );

    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command]
async fn cp_skill(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let serverid = format!("{}", msg.guild_id.unwrap().0);
    let userid = format!("{}", msg.author.id.0);
    let mut modifier: u32 = 0;
    let mut response: String = String::from("");
    if let Some(cid) = get_active_character(&serverid, &userid).await {
        if let Some(character) = get_character(cid).await {
            if let Ok(skill) = args.single::<String>() {
                let c: Vec<&Cp2020Skill> = character
                    .skills
                    .iter()
                    .filter(|&x| x.skill_name.to_lowercase() == skill.to_lowercase())
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
                    && skill.to_lowercase() == "awareness/notice"
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
    let user = if msg.is_private() {
        "You"
    } else {
        msg.author.name.as_str()
    };
    let result = d10_exploding();
    response = if result == 1 {
        format!("{}{} rolled a skill and critically failed!", response, user)
    } else {
        format!(
            "{}{} rolled a skill and got {}",
            response,
            user,
            result + modifier
        )
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command] //need to add a way to input a character.
async fn cp_add_char(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let mut result = "Not yet implemented".to_string();
    while let Ok(a) = _args.single::<String>() {
        result = result + "\n" + a.as_str();
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, result).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command]
async fn cp_pick_char(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let serverid = format!("{}", msg.guild_id.unwrap().0);
    let userid = format!("{}", msg.author.id.0);
    let characters = get_character_list(&serverid, &userid).await;
    let mut response = String::from("");
    if characters.len() == 0 {
        response += "You do not have any characters saved on this server!";
    } else if args.is_empty() {
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
        if let Ok(a) = args.single::<String>() {
            if let Ok(chr_num) = a.parse::<u32>() {
                let c = &characters[(chr_num - 1) as usize];
                if !set_active_character(&serverid, &userid, c.id).await {
                    response = String::from("Error setting character!");
                } else {
                    response += format!("Set active character to {} ({})", c.character_name, c.role)
                        .as_str()
                }
            } else {
                let chars: Vec<&Cp2020Character> = characters
                    .iter()
                    .filter(|&c| c.character_name.to_lowercase() == a.to_lowercase())
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
    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

// async fn add_character(interaction: Interaction, ctx: &Context) {
//     let modal = interaction
//         .application_command()
//         .unwrap()
//         .create_interaction_response(&ctx.http, |resp| {
//             resp.kind(InteractionResponseType::Modal)
//                 .interaction_response_data(|response| {
//                     response.custom_id("add_character");
//                     response.title("");
//                     response.components(|a_rows| {
//                         a_rows.create_action_row(|row| {
//                             row.create_input_text(|input| {
//                                 input
//                                     .custom_id("cname")
//                                     .style(InputTextStyle::Short)
//                                     .label("Name")
//                                     .required(true)
//                             })
//                         })
//                     })
//                 })
//         })
//         .await;
// }

async fn get_active_character(serverid: &str, userid: &str) -> Option<i64> {
    if let Some(db) = DB_POOL.get() {
        let sql = "SELECT character_id FROM active_characters WHERE server_id = ? AND user_id = ?";
        if let Ok(row) = sqlx::query(sql)
            .bind(serverid)
            .bind(userid)
            .fetch_one(db)
            .await
        {
            if let Ok(r) = row.try_get("character_id") {
                return Some(r);
            }
        }
    }
    None
}

async fn set_active_character(serverid: &str, userid: &str, characterid: i64) -> bool {
    if let Some(db) = DB_POOL.get() {
        let sql_del = "DELETE FROM active_characters WHERE server_id = ? AND user_id = ?";
        let sql_ins =
            "INSERT INTO active_characters (server_id, user_id, character_id) VALUES (?, ?, ?)";
        _ = sqlx::query(sql_del)
            .bind(serverid)
            .bind(userid)
            .execute(db)
            .await;
        let ret = sqlx::query(sql_ins)
            .bind(serverid)
            .bind(userid)
            .bind(characterid)
            .execute(db)
            .await;
        return ret.is_ok();
    }
    false
}

async fn get_character_list(serverid: &str, userid: &str) -> Vec<Cp2020Character> {
    let mut characters: Vec<Cp2020Character> = vec![];
    if let Some(db) = DB_POOL.get() {
        let sql = "SELECT id, character_name, role FROM cp2020.characters WHERE server_id = ? AND user_id = ? ORDER BY id ASC";
        let mut rows = sqlx::query(sql).bind(serverid).bind(userid).fetch(db);
        while let Ok(Some(chr)) = rows.try_next().await {
            let skills = get_skills(chr.get("id")).await;
            characters.push(Cp2020Character {
                id: chr.get("id"),
                character_name: chr.get("character_name"),
                role: chr.get("role"),
                inteligence: chr.get("inteligence"),
                reflex: chr.get("reflex"),
                initiative: chr.get("initiative"),
                tech: chr.get("tech"),
                cool: chr.get("cool"),
                attractiveness: chr.get("attractiveness"),
                movement: chr.get("movement"),
                body: chr.get("body"),
                empathy: chr.get("empathy"),
                special_ability: chr.get("special_ability"),
                skills: skills,
            });
        }
    }
    characters
}

fn d10_exploding() -> u32 {
    let mut total: u32 = 0;
    let mut done: bool = false;
    while !done {
        let d: u32 = rand::thread_rng().gen_range(1..=10);
        done = d != 10;
        total += d;
    }
    total
}

async fn get_character(characterid: i64) -> Option<Cp2020Character> {
    if let Some(db) = DB_POOL.get() {
        let character_qry = "SELECT id, server_id, user_id, role, inteligence, reflex, initiative, tech, cool, attractiveness, movement, body, empathy, special_ability 
                             FROM cp2020.characters
                             WHERE character_id = ?";
        let mut character = sqlx::query(character_qry).bind(characterid).fetch(db);
        if let Ok(Some(chr)) = character.try_next().await {
            let skills = get_skills(characterid).await;
            return Some(Cp2020Character {
                id: chr.get("id"),
                character_name: chr.get("character_name"),
                role: chr.get("role"),
                inteligence: chr.get("inteligence"),
                reflex: chr.get("reflex"),
                initiative: chr.get("initiative"),
                tech: chr.get("tech"),
                cool: chr.get("cool"),
                attractiveness: chr.get("attractiveness"),
                movement: chr.get("movement"),
                body: chr.get("body"),
                empathy: chr.get("empathy"),
                special_ability: chr.get("special_ability"),
                skills: skills,
            });
        }
    }
    None
}

async fn get_skills(characterid: i64) -> Vec<Cp2020Skill> {
    let mut skills: Vec<Cp2020Skill> = vec![];
    if let Some(db) = DB_POOL.get() {
        let skills_qry = "SELECT skill_name, skill_attribute, skill_value
                FROM cp2020.skills
                WHERE character_id = ?";
        let mut skill_rows = sqlx::query(skills_qry).bind(characterid).fetch(db);
        while let Ok(Some(r)) = skill_rows.try_next().await {
            skills.push(Cp2020Skill {
                skill_attribute: r.get("skill_attribute"),
                skill_name: r.get("skill_name"),
                skill_value: r.get("skill_value"),
            });
        }
    }
    skills
}

pub async fn cp2020_init() {
    if let Some(db) = DB_POOL.get() {
        let character_table = "CREATE TABLE IF NOT EXISTS cp2020.characters (
                        id INTEGER primary key AUTOINCREMENT,
                        server_id TEXT NOT NULL,
                        user_id TEXT NOT NULL,
                        character_name TEXT NOT NULL,
                        role TEXT NOT NULL,
                        inteligence INTEGER NOT NULL,
                        reflex INTEGER NOT NULL,
                        initiative INTEGER NOT NULL,
                        tech INTEGER NOT NULL,
                        cool INTEGER NOT NULL,
                        attractiveness INTEGER NOT NULL,
                        movement INTEGER NOT NULL,
                        body INTEGER NOT NULL,
                        empathy INTEGER NOT NULL,
                        special_ability INTEGER NOT NULL
                    )";
        let character_index = "CREATE INDEX IF NOT EXISTS cp2020.character_user_server_channel ON cp2020.characters (server_id, user_id)";
        let skill_table = "CREATE TABLE IF NOT EXISTS cp2020.skills (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        character_id INTEGER NOT NULL REFERENCES cp2020.characters (id) ON DELETE CASCADE ON UPDATE CASCADE,
                        skill_name TEXT NOT NULL,
                        skill_attribute TEXT NOT NULL,
                        skill_value INTEGER NOT NULL
                    )";
        let skill_index =
            "CREATE INDEX IF NOT EXISTS cp2020.skill_character_id ON cp2020.skills (character_id)";
        let active_character_table = "CREATE TABLE IF NOT EXISTS cp2020.active_characters (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        user_id TEXT NOT NULL,
                        server_id TEXT NOT NULL,
                        character_id INTEGER NOT NULL REFERENCES cp2020.characters (id) ON DELETE CASCADE ON UPDATE CASCADE
            )";
        let active_character_index = "CREATE UNIQUE INDEX IF NOT EXISTS cp2020.active_character_unique_server_user ON cp2020.active_characters (server_id, user_id)";
        _ = sqlx::query(character_table).execute(db).await;
        _ = sqlx::query(character_index).execute(db).await;
        _ = sqlx::query(skill_table).execute(db).await;
        _ = sqlx::query(skill_index).execute(db).await;
        _ = sqlx::query(active_character_table).execute(db).await;
        _ = sqlx::query(active_character_index).execute(db).await;
    }
}
