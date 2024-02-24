use crate::{ApplicationContext, Data, Error, BotConfig};
use poise::{
    serenity_prelude::{self as serenity, Channel, ChannelId, CreateMessage, Member, Mentionable},
    CreateReply, GuildRolesRef
};

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn create_role_menu(ctx: ApplicationContext<'_>, roles: GuildRolesRef) -> Result<(), Error> {
    let reply = format!("roles text: {:?}", roles);
    ctx.reply(reply);
    Ok(())
}
