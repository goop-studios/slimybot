use crate::{Error, ApplicationContext};
use poise::serenity_prelude::{self as serenity, CreateEmbed, CreateEmbedAuthor, CreateMessage, Mentionable, Channel};
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
pub async fn mkembed(ctx: ApplicationContext<'_>, #[description = "Channel in which the embed will be sent."] channel: Channel) -> Result<(), Error> {
    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("open_modal")
                .label("Create Embed")
                .style(serenity::ButtonStyle::Success),
        ])];

        let description = format!("This is a test of the embed modal, please click the button below to begin. it will write a message to the channel {}.", channel.mention());
    
        poise::CreateReply::default()
            .embed(CreateEmbed::default()
                .title("Embed Modal Test")
                .author(CreateEmbedAuthor::new("Goop Studios")
                    .url("https://goop-studios.monster"))
                .description(description))
            .components(components)
            .ephemeral(true)
    };

    ctx.send(reply).await?;
    
    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "open_modal")
        .await
    {
        let data =
            poise::execute_modal_on_component_interaction::<EmbedModal>(ctx, mci, None, None).await?;

        let embed = CreateEmbed::default()
            .title(format!("{}", data.as_ref().unwrap().first_input))
            .description(format!("{}", data.as_ref().unwrap().second_input))
            .author(CreateEmbedAuthor::new("Goop Studios")
                .url("https://goop-studios.monster"));

        let reply = poise::CreateReply::default()
            .content(format!("You entered: 1: {} 2: {}", data.as_ref().unwrap().first_input, data.as_ref().unwrap().second_input))
            .ephemeral(true);

        let embed_message = CreateMessage::default().embed(embed);

        channel.id().send_message(ctx.http(), embed_message).await?;
        ctx.send(reply).await?;
    }

    Ok(())
}
