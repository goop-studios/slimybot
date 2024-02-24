use std::ops::Deref;

use crate::{ApplicationContext, Data, Error, BotConfig};
use poise::{
    serenity_prelude::{self as serenity, Channel, ChannelId, CreateMessage, Member, Mentionable, Role},
    CreateReply,
};

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn create_role_menu(ctx: ApplicationContext<'_>, roles: Vec<Role>) -> Result<(), Error> {
    let reply = format!("{:?}", roles);
    ctx.reply(reply).await?;
    Ok(())
}