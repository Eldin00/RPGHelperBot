use serenity::{prelude::*, model::prelude::*, framework::standard::{CommandResult, macros::*}};

#[command]
async fn r(context: &Context, msg: &Message) -> CommandResult {
    println!("{}",msg.content);
    Ok(())
    
}