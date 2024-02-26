use crate::{
    config::parse::{AutoRole, Welcome},
    BotConfig, Data, Error,
};
use poise::serenity_prelude::{
    self as serenity, CacheHttp, CreateMessage, Member, Mentionable, ResumedEvent,
};
use std::path::Path;

pub async fn write_welcome(
    ctx: &serenity::Context,
    data: &Data,
    new_member: &Member,
) -> Result<(), Error> {
    if data.welcome.try_lock().expect("poisoned lock.").enabled {
        let welcome_channel = data
            .welcome
            .try_lock()
            .expect("poisoned lock.")
            .channel
            .expect("Welcome channel not set.");
        let welcome_channel = serenity::ChannelId::new(welcome_channel);
        let member_mention = new_member.mention();
        let server_name = new_member
            .guild_id
            .name(ctx)
            .expect("Server name not found.");
        let message: CreateMessage = CreateMessage::new().embed(
            serenity::CreateEmbed::default()
                .title("Welcome!")
                .description(format!("Welcome to {server_name}, {member_mention}!"))
                .color(0x00ff00),
        );
        welcome_channel.send_message(&ctx.http, message).await?;
    }
    Ok(())
}

pub async fn write_to_conf(
    _ctx: &serenity::Context,
    data: &Data,
    event: &ResumedEvent,
) -> Result<(), Error> {
    let (default_dir, welcome, autorole) = {
        (
            data.config_dir.try_lock().expect("Poisoned inner"),
            data.welcome.try_lock().expect("Poisoned lock."),
            data.autorole.try_lock().expect("Poisoned lock."),
        )
    };

    let config_data = BotConfig {
        welcome: Welcome {
            enabled: welcome.enabled,
            channel: welcome.channel.expect("Empty"),
        },
        autorole: AutoRole {
            enabled: autorole.enabled,
            role: autorole.role.expect("Empty"),
        },
    };

    let dir = default_dir.clone();
    let path = Path::new(&dir);

    config_data.write(path).expect("Can't write to path");

    println!(
        "Got event {:?}, writing to config, because uhhhhghghghhhhh",
        event
    );
    Ok(())
}

pub async fn set_role(
    ctx: &serenity::Context,
    data: &Data,
    new_member: &Member,
) -> Result<(), Error> {
    if data.autorole.try_lock().expect("poisoned lock.").enabled {
        let role = data
            .autorole
            .try_lock()
            .expect("poisoned lock.")
            .role
            .expect("Welcome channel not set.");
        let role_id = serenity::RoleId::new(role);
        new_member.add_role(&ctx.http(), role_id).await?;
    }
    Ok(())
}
