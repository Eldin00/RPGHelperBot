pub mod dice_commands {
 
    use serenity::{
        framework::standard::{macros::*, CommandResult, Args},
        model::channel::Message,
        prelude::*,
    };

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
    async fn roll(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
        if let Err(why) = msg.channel_id.say(&ctx.http, msg.content.as_str()).await {
            println!("Error sending message: {:?}", why);
        }
    
        Ok(())
    }
        

}
