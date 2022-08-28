pub mod cp2020_functions {
    use rand::Rng;
    use serenity::{
        framework::standard::{macros::*, Args, CommandResult},
        model::channel::Message,
        prelude::*,
    };

    #[group]
    #[commands(cp_init, cp_skill)]
    pub struct CpCommands;

    //These commands are minimally working placeholders. Eventually I want to add some form of character tracking

    #[command]
    async fn cp_init(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
        let user = if msg.is_private() {
            "You"
        } else {
            msg.author.name.as_str()
        };
        let result = d10_exploding();
        let response = format!("{} rolled for initiative and got {}", user, result);

        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            println!("Error sending message: {:?}", why);
        }
        Ok(())
    }

    #[command]
    async fn cp_skill(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
        let user = if msg.is_private() {
            "You"
        } else {
            msg.author.name.as_str()
        };
        let result = d10_exploding();
        let response = if result == 1 {
            format!("{} rolled a skill and critically failed!", user)
        } else {
            format!("{} rolled a skill and got {}", user, result)
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
}
