use std::path::Path;
use crate::{config::parse::{AutoRole, Welcome}, BotConfig, Data, Error};
use poise::serenity_prelude::{self as serenity, ResumedEvent};

pub async fn write_to_conf(
    _ctx: &serenity::Context,
    data: &Data,
    event: &ResumedEvent
) -> Result<(), Error> {
    let (
        default_dir, welcome, autorole
    ) = {
        (
            data.config_dir.try_lock().expect("Poisoned inner"),
            data.welcome.try_lock().expect("Poisoned lock."),
            data.autorole.try_lock().expect("Poisoned lock.")
        )
    };

    let config_data = BotConfig {
        welcome: Welcome {
            enabled: welcome.enabled,
            channel: welcome.channel.expect("Empty")
        },
        autorole: AutoRole {
            enabled: autorole.enabled,
            role: autorole.role.expect("Empty")
        }
    };
   
    let dir = default_dir.clone();
    let path = Path::new(&dir);

    config_data.write(path).expect("Can't write to path");

    println!("Got event {:?}, writing to config, because uhhhhghghghhhhh", event);
    Ok(())
}
