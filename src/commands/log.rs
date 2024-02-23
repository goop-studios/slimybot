use crate::{Error, ApplicationContext};
use poise::{serenity_prelude::{self as serenity, Channel, ChannelId, CreateEmbed, CreateEmbedAuthor, CreateMessage, Mentionable}, CreateReply};

#[poise::command(slash_command, required_permissions="ADMINISTRATOR")]
pub async fn enable_welcome(ctx: ApplicationContext<'_>, #[description = "Channel which welcome messages will be sent."] channel: Channel) -> Result<(), Error> {
    let welcome_channel = {
        let mut welcome_channel = ctx.data().welcome_channel.lock().expect("poisoned lock");
        *welcome_channel = Some(channel.id().into());
        *welcome_channel
    };

    let send_welcome_message = {
        let mut send_welcome_message = ctx.data().send_welcome_message.lock().expect("poisoned lock");
        *send_welcome_message = true;
        *send_welcome_message
    };
    
    let channel_mention = ChannelId::new(welcome_channel.unwrap()).mention();

    let reply = CreateReply::default()
        .content(format!("Welcome message has been set to {send_welcome_message} and the welcome channel has been set to {channel_mention}!"))
        .ephemeral(true);
    ctx.send(reply).await?;
    Ok(())
}