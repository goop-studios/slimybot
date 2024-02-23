use crate::{Error, ApplicationContext, Data};
use poise::{serenity_prelude::{self as serenity, Channel, ChannelId, Member, Mentionable}, CreateReply};

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

pub async fn write_welcome(ctx: &serenity::Context, data: &Data, new_member: &Member) -> Result<(), Error> {
    if *data.send_welcome_message.try_lock().expect("poisoned lock.") {
        let welcome_channel = data.welcome_channel.try_lock().expect("poisoned lock.").expect("Welcome channel not set.");
        let welcome_channel = serenity::ChannelId::new(welcome_channel);
        let welcome_message = format!("Welcome to the server, {}!", new_member.user.mention());
        welcome_channel.say(&ctx.http, welcome_message).await?;
    }
    Ok(())
}