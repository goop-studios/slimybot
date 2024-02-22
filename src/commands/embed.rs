use crate::{Error, Context, Data};
use poise::serenity_prelude as serenity;
use serenity::Channel;
use poise::Modal;

type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>; 

#[derive(Debug, Modal)]
#[name = "Modal test"]
struct EmbedModal {
    #[name = "Title"]
    #[placeholder = "Input here"]
    #[min_length = 3]
    #[max_length = 500]
    first_input: String,
    #[name = "Body"]
    #[paragraph]
    second_input: String,
}


/// Creates a modal for testing 
#[poise::command(slash_command, required_permissions="ADMINISTRATOR")]
pub async fn mkembed(ctx: ApplicationContext<'_>, _channel: Channel) -> Result<(), Error> {
    let gotten_data = EmbedModal::execute(ctx).await?;
    let reply = format!("got data: {:?}", gotten_data);
    ctx.reply(reply).await?; 
    Ok(())
}
