pub mod dice_commands {
    use rand::Rng;
    use regex::{Match, Regex};
    use serenity::{
        framework::standard::{macros::*, Args, CommandResult},
        model::channel::Message,
        prelude::*,
    };
    use std::cmp::max;

    #[group]
    #[commands("roll", "ping")]
    pub struct General;

    #[command]
    async fn ping(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
            println!("Error sending message: {:?}", why);
        }

        Ok(())
    }

    #[command]
    #[aliases("r")]
    async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
        let m = eval_dice_command(msg, args);
        match m {
            Some(m) => {
                _ = msg.channel_id.say(&ctx.http, m).await;
            }
            None => {
                _ = msg
                    .channel_id
                    .say(&ctx.http, "Error parsing dice roll.")
                    .await;
            }
        }
        Ok(())
    }

    fn parse_int(value: Option<Match>, default: i32) -> i32 {
        match value {
            Some(value) => value.as_str().parse::<i32>().unwrap_or(default),
            None => default,
        }
    }

    pub fn eval_dice_command(msg: &Message, args: Args) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?xi)^(?:(?P<die_count>\d+)?d(?P<die_sides>\d+))?((?P<modifier>[+-]\d+)|((\*)(?P<repeat>\d+)))*$"
            )
            .unwrap();
        }
        if let Some(group) = RE.captures(&args.message().replace(" ", "")) {
            if group.name("die_sides").is_some() {
                let die_count: i32 = max(parse_int(group.name("die_count"), 1), 1);
                let die_sides: i32 = max(parse_int(group.name("die_sides"), 10), 1);
                let modifier: i32 = parse_int(group.name("modifier"), 0);
                let repeat: i32 = parse_int(group.name("repeat"), 1);
                let mut canonical_cmd = format!("{}D{}", die_count, die_sides);

                if modifier > 0 {canonical_cmd = format!("{} + {}", canonical_cmd, modifier); } 
                else if modifier < 0 {canonical_cmd = format!("{} - {}", canonical_cmd, modifier*-1);}
                if repeat > 1 {canonical_cmd = format!("{}, {} times.", canonical_cmd, repeat);}  


                let mut result: String = "".to_string();
                for _i in 0..repeat {
                    result = format!("{}{} ", result, (xdy(die_count, die_sides) + modifier));
                }
                let user = if msg.is_private() {"You"} else {msg.author.name.as_str() };
                result = format!{"{} rolled {} and got:\n{}", user, canonical_cmd, result};
                return Some(result);
            }
        }
        None
    }

    fn xdy(x: i32, y: i32) -> i32 {
        let mut total: i32 = 0;
        for _i in 1..=x {
            total += rand::thread_rng().gen_range(1..=y);
        }
        total
    }
}
