use anyhow::Context as _;
use poise::{
    samples::HelpConfiguration,
    serenity_prelude::{ClientBuilder, GatewayIntents},
};
use shuttle_secrets::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use commands::{
    embed::mkembed,
    log::{set_welcome, toggle_welcome},
    moderators::{ban, kick, mute, purge, unban},
    roles::{set_autorole, set_reaction_role, toggle_autorole},
};

use eventhandler::handle_event;

use state::{AutoRole, Data, Welcome};

use config::parse::BotConfig;

mod commands;
mod config;
mod eventhandler;
pub(crate) mod state;

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
            welcome: Arc::new(Mutex::new(Welcome {
                enabled: config.welcome.enabled,
                channel: Some(config.welcome.channel),
            })),
            autorole: Arc::new(Mutex::new(AutoRole {
                enabled: config.autorole.enabled,
                role: Some(config.autorole.role),
            })),
            reaction_roles: Arc::new(Mutex::new(Default::default())),
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
                kick(),
                ban(),
                unban(),
                mute(),
                // Roles
                set_autorole(),
                toggle_autorole(),
                set_reaction_role(),
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
