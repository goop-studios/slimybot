use std::sync::Arc;

use crate::{state::ReactionParams, ApplicationContext, Error};
use poise::{
    serenity_prelude::{self as serenity, Message, ReactionType, Role, RoleId},
    CreateReply,
};

/// Set autorole role.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Roles"
)]
pub async fn set_autorole(
    ctx: ApplicationContext<'_>,
    #[description = "Role that will be applied on member join."] role: Role,
) -> Result<(), Error> {
    let role = {
        let mut data = ctx.data().autorole.try_lock()?;
        data.role = Some(role.id.get());
        data.role
    };

    let role = if let Some(role) = role {
        role
    } else {
        panic!("Can't find role.");
    };

    let role_name = if let Some(role_name) = RoleId::new(role).to_role_cached(ctx) {
        role_name
    } else {
        panic!("Failed to find role");
    };

    let reply = CreateReply::default()
        .content(format!("Set the autorole to {role_name}"))
        .ephemeral(true);

    ctx.send(reply).await?;
    Ok(())
}
/// Toggle whether autorole should be on or off.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Roles"
)]
pub async fn toggle_autorole(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let enabled = {
        let mut data = ctx.data().autorole.try_lock()?;
        data.enabled = !data.enabled;
        data.enabled
    };

    let reply = CreateReply::default()
        .content(format!("Set autorole to {enabled}"))
        .ephemeral(true);
    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Roles"
)]
pub async fn set_reaction_role(
    ctx: ApplicationContext<'_>,
    #[description = "Emoji to react with."] emoji: ReactionType,
    #[description = "Role to give on reaction."] role: Role,
    message: Message,
) -> Result<(), Error> {
    let data = Arc::clone(&ctx.data().reaction_roles);

    let mut data = data.try_lock().expect("Poisoned lock.");

    let new_emoji = emoji.clone();
    let message_clone = message.clone();
    message.delete_reactions(ctx.http()).await?;
    message.react(ctx.http(), new_emoji.clone()).await?;

    if let Some(index) = data
        .reactions
        .iter()
        .position(|x| x.message.id == message.id && x.message.channel_id == message.channel_id)
    {
        data.reactions[index] = ReactionParams {
            role: role.id.get(),
            emoji,
            message,
        };
    } else {
        data.add_reaction(role.id.get(), emoji, message);
    }

    let guild_id = ctx.guild_id().expect("Guild not found.");

    let link = format!(
        "https://discord.com/channels/{}/{}/{}",
        guild_id.get(),
        ctx.channel_id().get(),
        message_clone.id.get()
    );

    let reply = CreateReply::default()
        .embed(
            serenity::CreateEmbed::default()
                .description(format!(
                    "Set reaction role for {} with emoji {}",
                    role,
                    new_emoji.to_string()
                ))
                .field("Message:", link, false),
        )
        .ephemeral(true);

    let debug = format!("{:?}", data);

    ctx.reply(debug).await?;
    ctx.send(reply).await?;
    Ok(())
}
