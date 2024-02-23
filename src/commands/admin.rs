use crate::{Context, Error};
use poise::serenity_prelude::GetMessages;

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
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

    ctx.reply("Purging messages!").await?;
    Ok(())
}
