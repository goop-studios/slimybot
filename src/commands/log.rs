use crate::{ApplicationContext, Error};
use poise::{
    serenity_prelude::{Channel, ChannelId, Mentionable},
    CreateReply,
};

/// Set whether the bot should send a welcome message in picked channel.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Logs"
)]
pub async fn toggle_welcome(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let enabled = {
        let mut welcome = ctx.data().welcome.try_lock().expect("Poisoned lock.");
        welcome.enabled = !welcome.enabled;
        welcome.enabled
    };

    let reply = CreateReply::default()
        .content(format!("Welcome has been set to {enabled}"))
        .ephemeral(true);

    ctx.send(reply).await?;
    Ok(())
}

/// Set channel the bot shall welcome people in.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Logs"
)]
pub async fn set_welcome(
    ctx: ApplicationContext<'_>,
    #[description = "Channel which welcome messages will be sent."] channel: Channel,
) -> Result<(), Error> {
    let channel = {
        let mut welcome = ctx.data().welcome.try_lock().expect("Poisoned lock.");
        welcome.channel = Some(channel.id().get());
        welcome.channel
    };
    let channel_mention = ChannelId::new(channel.unwrap()).mention();

    let reply = CreateReply::default()
        .content(format!("Welcome channel has ben set to {channel_mention}"))
        .ephemeral(true);

    ctx.send(reply).await?;
    Ok(())
}
