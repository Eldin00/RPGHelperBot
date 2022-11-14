use rand::Rng;
use sqlx::Row;

use crate::dbinterface::DB_POOL;

pub struct Cp2020Skill {
    pub skill_name: String,
    pub skill_attribute: String,
    pub skill_value: u8,
}

pub struct Cp2020Character {
    pub id: i64,
    pub character_name: String,
    pub role: String,
    pub inteligence: u8,
    pub reflex: u8,
    pub initiative: u8,
    pub tech: u8,
    pub cool: u8,
    pub attractiveness: u8,
    pub movement: u8,
    pub body: u8,
    pub empathy: u8,
    pub special_ability: u8,
    pub skills: Vec<Cp2020Skill>,
}

// async fn add_character(interaction: Interaction, ctx: &Context) {
//     let modal = interaction.application_command().unwrap()
//     .create_interaction_response(&ctx.http, |resp| {
//         resp.kind(InteractionResponseType::Modal)
//             .interaction_response_data(|response| {
//                 response.custom_id("add_character");
//                 response.title("");
//                 response.components(|a_rows| {
//                     a_rows.create_action_row(|row| {
//                         row.create_input_text(|input| {
//                             input.custom_id("cname")
//                             .style(InputTextStyle::Short)
//                             .label("Name")
//                             .required(true)
//                         })
//                     })
//                 })
//             })
//     }).await;
// }

pub async fn get_active_character(serverid: &str, userid: &str) -> Option<i64> {
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
        while let Ok(Some(chr)) = serenity::futures::TryStreamExt::try_next(&mut rows).await {
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

pub fn d10_exploding() -> u32 {
    let mut total: u32 = 0;
    let mut done: bool = false;
    while !done {
        let d: u32 = rand::thread_rng().gen_range(1..=10);
        done = d != 10;
        total += d;
    }
    total
}

pub async fn get_character(characterid: i64) -> Option<Cp2020Character> {
    if let Some(db) = DB_POOL.get() {
        let character_qry = "SELECT id, server_id, user_id, role, inteligence, reflex, initiative, tech, cool, attractiveness, movement, body, empathy, special_ability 
                             FROM cp2020.characters
                             WHERE character_id = ?";
        let mut character = sqlx::query(character_qry).bind(characterid).fetch(db);
        if let Ok(Some(chr)) = serenity::futures::TryStreamExt::try_next(&mut character).await {
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
        while let Ok(Some(r)) = serenity::futures::TryStreamExt::try_next(&mut skill_rows).await {
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
