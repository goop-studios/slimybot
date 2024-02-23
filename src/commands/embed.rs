use std::vec;

use crate::{Error, ApplicationContext};
use poise::serenity_prelude::{self as serenity, CreateEmbed, CreateEmbedAuthor, Mentionable};
use serenity::Channel;
use poise::Modal;

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
    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("open_modal")
                .label("Create Embed")
                .style(serenity::ButtonStyle::Success),
        ])];

        let description = format!("This is a test of the embed modal, please click the button below to begin. it will write a message to the channel {}.", _channel.mention());
    
        poise::CreateReply::default()
            .embed(CreateEmbed::default()
                .title("Embed Modal Test")
                .author(CreateEmbedAuthor::new("Goop Studios")
                    .url("https://goop-studios.monster"))
                .description(description))
            .components(components)
    };

    ctx.send(reply).await?;
    
    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "open_modal")
        .await
    {
        let data =
            poise::execute_modal_on_component_interaction::<EmbedModal>(ctx, mci, None, None).await?;
        ctx.reply(format!("data gotten:  {:?}", data)).await?;
    }


    Ok(())
}
