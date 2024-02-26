use anyhow::Context as _;
use poise::{
    samples::HelpConfiguration,
    serenity_prelude::{self as serenity, ClientBuilder, GatewayIntents},
};
use shuttle_secrets::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use std::path::Path;
use std::sync::Mutex;

use commands::{
    embed::mkembed,
    log::{set_welcome, toggle_welcome},
    moderators::purge,
    roles::{set_autorole, toggle_autorole},
};

use eventhandler::{set_role, write_to_conf, write_welcome};

use state::{AutoRole, Data, Welcome};

use config::parse::BotConfig;

mod commands;
mod config;
mod eventhandler;
mod state;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

/// Show help for a single command or all commands.
#[poise::command(slash_command, track_edits, category = "Utility")]
async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for."] mut command: Option<String>,
) -> Result<(), Error> {
    if ctx.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c)),
            None => Some(ctx.invoked_command_name().to_string()),
        };
    }

    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        extra_text_at_bottom: "Made with <3 by h4rl @ goop-studios.monster",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

async fn handle_event(
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
        _ => {}
    }
    Ok(())
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let default_path = Path::new("config.toml");

    let bot_state = if let Ok(config) = BotConfig::read(&default_path) {
        println!("{:?}", config);
        Data {
            welcome: Mutex::new(Welcome {
                enabled: config.welcome.enabled,
                channel: Some(config.welcome.channel),
            }),
            autorole: Mutex::new(AutoRole {
                enabled: config.autorole.enabled,
                role: Some(config.autorole.role),
            }),
            ..Default::default()
        }
    } else {
        Data::default()
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // Utility
                help(),
                // Embed
                mkembed(),
                // Logs
                set_welcome(),
                toggle_welcome(),
                // Moderation
                purge(),
                // Roles
                set_autorole(),
                toggle_autorole(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(handle_event(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(bot_state)
            })
        })
        .build();

    let client = ClientBuilder::new(
        discord_token,
        GatewayIntents::non_privileged()
            | GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILD_PRESENCES,
    )
    .framework(framework)
    .await
    .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
