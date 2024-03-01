use crate::{
    config::parse::{AutoRole, Welcome},
    BotConfig, Data, Error,
};
use poise::serenity_prelude::{
    self as serenity, CacheHttp, CreateMessage, Member, Mentionable, ResumedEvent,
};
use std::path::Path;
use tracing::info;

async fn write_welcome(
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

async fn write_to_conf(
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

async fn set_role(ctx: &serenity::Context, data: &Data, new_member: &Member) -> Result<(), Error> {
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

pub async fn handle_event(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            write_welcome(ctx, data, new_member).await?;
            set_role(ctx, data, new_member).await?;
        }
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("{} is ready!", data_about_bot.user.name);
        }
        serenity::FullEvent::Resume { event } => {
            write_to_conf(ctx, data, event).await?;
        }
        serenity::FullEvent::ReactionAdd { add_reaction } => {
            let data = data.reaction_roles.try_lock().expect("Poisoned lock.");

            info!("Reaction added: {:?}", add_reaction);

            let add_reaction_clone = add_reaction.clone();
            if add_reaction_clone.member.unwrap().user.id != ctx.cache().unwrap().current_user().id
            {
                for reaction in &data.reactions {
                    if reaction.message.id == add_reaction.message_id {
                        let role_id = serenity::RoleId::new(reaction.role);
                        info!("Adding role {:?}", role_id.to_role_cached(&ctx.cache));
                        add_reaction
                            .member
                            .as_ref()
                            .expect("Member not found.")
                            .add_role(&ctx.http(), role_id)
                            .await?;

                        info!(
                            "add_reaction member: {:?}",
                            add_reaction.member.as_ref().unwrap().user.id
                        );
                    }
                }
            }
        }
        serenity::FullEvent::ReactionRemove { removed_reaction } => {
            let data = data.reaction_roles.try_lock().expect("Poisoned lock.");

            info!("Reaction removed: {:?}", removed_reaction);

            for reaction in &data.reactions {
                if reaction.message.id == removed_reaction.message_id {
                    let role_id = serenity::RoleId::new(reaction.role);
                    info!("Removing role {:?}", role_id.to_role_cached(&ctx.cache));
                    removed_reaction
                        .member
                        .as_ref()
                        .expect("Member not found.")
                        .remove_role(&ctx.http(), role_id)
                        .await?;
                    info!(
                        "removed_reaction member: {:?}",
                        removed_reaction.member.as_ref().unwrap().user.id
                    );
                } else {
                    info!(
                        "Reaction not found.. {} != {}",
                        reaction.message.id, removed_reaction.message_id
                    );
                }
            }
        }
        _ => {}
    }
    Ok(())
}
