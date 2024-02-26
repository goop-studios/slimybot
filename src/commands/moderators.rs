use crate::{Context, Error};
use poise::{
    serenity_prelude::{self as serenity, EditRole, GetMessages, Mentionable, PermissionOverwrite},
    CreateReply,
};
/// Delete a number of messages from the channel.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Moderation"
)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "Number of messages to delete."] amount: u8,
) -> Result<(), Error> {
    let messages = ctx
        .channel_id()
        .messages(&ctx.http(), GetMessages::new().limit(amount))
        .await?
        .iter()
        .map(|m| m.id)
        .collect::<Vec<_>>();
    ctx.channel_id()
        .delete_messages(&ctx.http(), messages)
        .await?;

    let reply = CreateReply::default()
        .embed(
            serenity::CreateEmbed::default().description(format!("Deleted {} messages.", amount)),
        )
        .ephemeral(true);

    ctx.send(reply).await?;
    Ok(())
}
/// Kick a user from the server.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Moderation"
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "User to kick."] user: serenity::Member,
    #[description = "Reason for kicking."] reason: Option<String>,
) -> Result<(), Error> {
    user.kick_with_reason(&ctx.http(), &reason.unwrap_or_default())
        .await?;

    let reply = CreateReply::default()
        .embed(
            serenity::CreateEmbed::default()
                .description(format!("Kicked user {}.", user.mention())),
        )
        .ephemeral(true);

    ctx.send(reply).await?;
    Ok(())
}

/// Ban a user from the server.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Moderation"
)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "User to ban."] user: serenity::Member,
    #[description = "Reason for banning."] reason: Option<String>,
) -> Result<(), Error> {
    user.ban_with_reason(&ctx.http(), 0, &reason.unwrap_or_default())
        .await?;

    let reply = CreateReply::default()
        .embed(
            serenity::CreateEmbed::default()
                .description(format!("Banned user {}.", user.mention())),
        )
        .ephemeral(true);

    ctx.send(reply).await?;
    Ok(())
}
/// Unban a user from the server.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Moderation"
)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "User to unban."] user: serenity::User,
) -> Result<(), Error> {
    ctx.guild_id().unwrap().unban(&ctx.http(), &user).await?;

    let reply = CreateReply::default()
        .embed(
            serenity::CreateEmbed::default()
                .description(format!("Unbanned user {}.", user.mention())),
        )
        .ephemeral(true);

    ctx.send(reply).await?;
    Ok(())
}

/// Mute a user in the server.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Moderation"
)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "User to mute."] user: serenity::Member,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Guild not found.");
    let guild = ctx.partial_guild().await.expect("Guild not found.");
    let builder = EditRole::default().name("Muted");

    let role = if let Some(role) = guild.role_by_name("Muted") {
        role.clone()
    } else {
        let new_role = guild_id.create_role(&ctx.http(), builder).await?;
        new_role
    };

    for channel in ctx.guild_id().unwrap().channels(&ctx.http()).await? {
        let channel = channel.1;
        let perms = PermissionOverwrite {
            allow: serenity::Permissions::empty(),
            deny: serenity::Permissions::SPEAK | serenity::Permissions::SEND_MESSAGES,
            kind: serenity::PermissionOverwriteType::Role(role.id),
        };
        channel
            .create_permission(&ctx.http(), perms)
            .await
            .map_err(|e| Box::new(e) as Error)?;
    }

    user.add_role(&ctx.http(), role.id).await?;

    let embed = serenity::CreateEmbed::default()
        .description(format!("Muted user {}.", user.mention()))
        .color(serenity::Colour::DARK_RED);

    let reply = CreateReply::default().embed(embed).ephemeral(true);
    ctx.send(reply).await?;
    Ok(())
}
