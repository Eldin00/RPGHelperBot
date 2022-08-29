pub mod cp2020_functions {
    use rand::Rng;
    //    use sqlx::Sqlite::SqlitePool;
    use serenity::{
        framework::standard::{macros::*, Args, CommandResult},
        model::channel::Message,
        prelude::*,
    };

    use crate::dbinterface::dbinterface::DB_POOL;

    struct Cp2020Skill {
        skill_name: String,
        skill_attribute: String,
        skill_value: u8,
    }

    struct Cp2020Character {
        servername: String,
        username: String,
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
    #[commands(cp_init, cp_skill)]
    pub struct CpCommands;

    //These commands are minimally working placeholders. Eventually I want to add some form of character tracking

    #[command]
    async fn cp_init(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
        let serverid = format!("{}", msg.guild_id.unwrap().0);
        let userid = format!("{}", msg.author.id.0);
        let mut character_found = false;
        let modifier: u32 = match get_character(serverid.as_str(), userid.as_str()).await {
            Some(character) => {
                character_found = true;
                0
            }
            _ => 0,
        };
        let user = if msg.is_private() {
            "You"
        } else {
            msg.author.name.as_str()
        };
        let result = d10_exploding();
        let response_modifier = if character_found {
            "with character modifiers"
        } else {
            "without character modifiers"
        };
        let response = format!(
            "{} rolled for initiative {} and got:\n {}",
            user,
            response_modifier,
            result + modifier
        );

        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            println!("Error sending message: {:?}", why);
        }
        Ok(())
    }

    #[command]
    async fn cp_skill(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
        let serverid = format!("{}", msg.guild_id.unwrap().0);
        let userid = format!("{}", msg.author.id.0);
        let modifier: u32 = match get_character(serverid.as_str(), userid.as_str()).await {
            Some(character) => 0,
            _ => 0,
        };
        let user = if msg.is_private() {
            "You"
        } else {
            msg.author.name.as_str()
        };
        let result = d10_exploding();
        let response = if result == 1 {
            format!("{} rolled a skill and critically failed!", user)
        } else {
            format!("{} rolled a skill and got {}", user, result + modifier)
        };

        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            println!("Error sending message: {:?}", why);
        }
        Ok(())
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

    async fn get_character(serverid: &str, userid: &str) -> Option<Cp2020Character> {
        // Stub. Returning none just to make it compile.
        None
    }

    pub async fn cp2020_init() {
        loop {
            if let Ok(db) = DB_POOL.try_read().as_deref() {
                match db {
                    Some(pool) => {
                        let character_table = "CREATE TABLE IF NOT EXISTS cp2020.characters (
                        id INTEGER primary key AUTOINCREMENT,
                        server_id TEXT NOT NULL,
                        user_id TEXT NOT NULL,
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
                        let character_table_index = "CREATE INDEX IF NOT EXISTS cp2020.character_unique_user_server_channel ON cp2020.characters (server_id, user_id)";
                        let skill_table = "CREATE TABLE IF NOT EXISTS cp2020.skills (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        character_id INTEGER NOT NULL REFERENCES cp2020.characters (id) ON DELETE CASCADE ON UPDATE CASCADE,
                        skill_name TEXT NOT NULL,
                        skill_attribute TEXT NOT NULL,
                        skill_value INTEGER NOT NULL
                    )";
                        _ = sqlx::query(character_table).execute(pool).await;
                        _ = sqlx::query(character_table_index).execute(pool).await;
                        _ = sqlx::query(skill_table).execute(pool).await;
                    }
                    None => {}
                }
                break;
            }
        }
    }
}
