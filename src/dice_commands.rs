pub mod dice_commands {

    use serenity::{
        framework::standard::{macros::*, CommandResult},
        model::prelude::*,
        prelude::*,
    };

    #[command]
    async fn r(context: &Context, msg: &Message) -> CommandResult {
        if let Err(why) = msg
            .channel_id
            .say(&context.http, msg.content.as_str())
            .await
        {
            println!("Error sending message: {}", why);
        }
        Ok(())
    }
}
