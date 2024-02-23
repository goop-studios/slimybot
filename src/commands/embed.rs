use std::vec;

use crate::{Error, ApplicationContext};
use poise::serenity_prelude::{self as serenity, CreateEmbed, CreateEmbedAuthor};
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
    
        poise::CreateReply::default()
            .embed(CreateEmbed::default()
                .title("Embed Modal Test")
                .author(CreateEmbedAuthor::new("Goop Studios")
                    .url("https://goop-studios.monster"))
                .description("This is a test of the embed modal."))
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
        println!("Got data: {:?}", data);
    }


    Ok(())
}
