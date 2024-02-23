use anyhow::Context as _;
use std::sync::Mutex;
use poise::serenity_prelude::{self as serenity, ClientBuilder, GatewayIntents, Mentionable};
use shuttle_secrets::SecretStore;
use shuttle_serenity::ShuttleSerenity;

use commands::{embed::mkembed, log::{enable_welcome, write_welcome}};

mod commands;

pub struct Data {
    welcome_channel: Mutex<Option<u64>>,
    send_welcome_message: Mutex<bool>,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>; 

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}


async fn handle_event(ctx: &serenity::Context, event: &serenity::FullEvent, _framework: poise::FrameworkContext<'_, Data, Error>, data: &Data,) -> Result<(), Error> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => write_welcome(ctx, data, new_member).await?,
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("{} is ready!", data_about_bot.user.name);
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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), mkembed(), enable_welcome()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(handle_event(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    welcome_channel: Mutex::new(None),
                    send_welcome_message: Mutex::new(false),
                })
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::all())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
