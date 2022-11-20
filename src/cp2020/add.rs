use serenity::{
    builder::CreateApplicationCommand, model::prelude::application::interaction::Interaction,
    prelude::*,
};

use crate::cp2020::common::*;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("add").description("Add a character")
}

pub async fn run(interaction: &Interaction, ctx: &Context) {
    //code to add a character will go here.
}
