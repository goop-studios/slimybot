use crate::{ApplicationContext, Error};
use poise::serenity_prelude::{
    self as serenity, Channel, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage,
    Mentionable,
};
use poise::Modal;

#[derive(Debug, Modal, Clone)]
#[name = "Create Embed - Content"]
struct EmbedModal1 {
    #[name = "Title"]
    #[placeholder = "Sample text"]
    #[min_length = 3]
    #[max_length = 200]
    title: String,

    #[name = "Body"]
    #[paragraph]
    description: String,

    #[name = "Footer"]
    #[placeholder = "Sample text"]
    footer: Option<String>,

    #[name = "Author"]
    #[placeholder = "Sample text"]
    author: Option<String>,
}

#[derive(Debug, Modal, Clone)]
#[name = "Create Embed - Urls and Customization"]
struct EmbedModal2 {
    #[name = "Embed URL"]
    #[placeholder = "https://example.com"]
    url: Option<String>,

    #[name = "Embed Color"]
    color: Option<String>,

    #[name = "Image URL"]
    #[placeholder = "https://example.com/example.jpeg"]
    image: Option<String>,

    #[name = "Thumbnail URL"]
    #[placeholder = "https://example.com/example.jpeg"]
    thumbnail: Option<String>,
}

#[derive(Debug, Modal, Clone)]
#[name = "Create Embed - Fields"]
struct EmbedModal3 {
    #[name = "Field Name"]
    #[placeholder = "Sample text"]
    name: String,

    #[name = "Field Body"]
    #[placeholder = "Sample text"]
    body: String,

    #[name = "Inline"]
    #[placeholder = "true/false"]
    is_inline: String,
}

struct CombinedEmbed {
    data1: Option<EmbedModal1>,
    data2: Option<EmbedModal2>,
    fields: Option<Vec<EmbedModal3>>,
}

/// Creates a modal for making embeds in picked channel. WIP
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Embed"
)]
pub async fn mkembed(
    ctx: ApplicationContext<'_>,
    #[description = "Channel in which the embed will be sent."] channel: Channel,
) -> Result<(), Error> {
    let reply = {
        let components = vec![
            serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("open_modal")
                    .label("Create Embed - Content")
                    .style(serenity::ButtonStyle::Primary),
                serenity::CreateButton::new("open_modal1")
                    .label("Create Embed - Urls and Customization")
                    .style(serenity::ButtonStyle::Secondary),
            ]),
            serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("add_field")
                    .label("Add field")
                    .style(serenity::ButtonStyle::Success),
                serenity::CreateButton::new("remove_field")
                    .label("Remove field")
                    .style(serenity::ButtonStyle::Danger),
            ]),
            serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("cancel")
                    .label("Cancel")
                    .style(serenity::ButtonStyle::Danger),
                serenity::CreateButton::new("submit")
                    .label("Submit")
                    .style(serenity::ButtonStyle::Success),
            ]),
        ];

        let description = format!("This is a test of the embed modal, please click the button below to begin. it will write a message to the channel {}.", channel.mention());

        poise::CreateReply::default()
            .embed(
                CreateEmbed::default()
                    .title("Embed Modal Test")
                    .author(
                        CreateEmbedAuthor::new("Goop Studios").url("https://goop-studios.monster"),
                    )
                    .description(description),
            )
            .components(components)
            .ephemeral(true)
    };

    ctx.send(reply).await?;

    let mut combined_data = CombinedEmbed {
        data1: None,
        data2: None,
        fields: None,
    };

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .await
    {
        match mci.data.custom_id.as_str() {
            "open_modal" => {
                let data = if let Some(data) = poise::execute_modal_on_component_interaction::<
                    EmbedModal1,
                >(ctx, mci, None, None)
                .await?
                {
                    data
                } else {
                    continue;
                };

                let reply_data = data.clone();
                let reply = poise::CreateReply::default()
                    .content(format!(
                        "You entered: \nTitle: {} \nBody: {} \nFooter: {} \nAuthor: {}",
                        reply_data.title,
                        reply_data.description,
                        reply_data.footer.unwrap_or_else(|| "None".to_string()),
                        reply_data.author.unwrap_or_else(|| "None".to_string()),
                    ))
                    .ephemeral(true);

                ctx.send(reply).await?;
                combined_data.data1 = Some(data);
            }
            "open_modal1" => {
                let data = if let Some(data) = poise::execute_modal_on_component_interaction::<
                    EmbedModal2,
                >(ctx, mci, None, None)
                .await?
                {
                    data
                } else {
                    continue;
                };

                let reply_data = data.clone();
                let reply = poise::CreateReply::default()
                    .content(format!(
                        "You entered: \nURL: {} \nColor: {} \nImage URL: {} \nThumbnail URL: {}",
                        reply_data.url.unwrap_or_else(|| "None".to_string()),
                        reply_data.color.unwrap_or_else(|| "None".to_string()),
                        reply_data.image.unwrap_or_else(|| "None".to_string()),
                        reply_data.thumbnail.unwrap_or_else(|| "None".to_string()),
                    ))
                    .ephemeral(true);
                ctx.send(reply).await?;
                combined_data.data2 = Some(data);
            }
            "add_field" => {
                let data = if let Some(data) = poise::execute_modal_on_component_interaction::<
                    EmbedModal3,
                >(ctx, mci, None, None)
                .await?
                {
                    data
                } else {
                    continue;
                };

                let reply_data = data.clone();
                let reply = poise::CreateReply::default()
                    .content(format!(
                        "You entered: \nField Name: {} \nField Body: {} \nInline: {}",
                        reply_data.name, reply_data.body, reply_data.is_inline,
                    ))
                    .ephemeral(true);
                ctx.send(reply).await?;
                if let Some(fields) = &mut combined_data.fields {
                    fields.push(data);
                } else {
                    combined_data.fields = Some(vec![data]);
                }
            }
            "remove_field" => {
                if let Some(fields) = &mut combined_data.fields {
                    fields.pop();
                } else {
                    let reply = poise::CreateReply::default()
                        .content("No fields to remove.")
                        .ephemeral(true);
                    ctx.send(reply).await?;
                }
            }
            "cancel" => {
                let reply = poise::CreateReply::default()
                    .content("Cancelled.")
                    .ephemeral(true);
                ctx.send(reply).await?;
                return Ok(());
            }
            "submit" => {
                let reply = poise::CreateReply::default()
                    .content("Submitting...")
                    .ephemeral(true);
                ctx.send(reply).await?;
                break;
            }
            _ => {}
        }
    }

    let data1 = if let Some(data1) = combined_data.data1 {
        data1
    } else {
        EmbedModal1 {
            title: "None".to_string(),
            description: "None".to_string(),
            footer: None,
            author: None,
        }
    };

    let data2 = if let Some(data2) = combined_data.data2 {
        data2
    } else {
        EmbedModal2 {
            url: None,
            color: None,
            image: None,
            thumbnail: None,
        }
    };

    let embed_fields = if let Some(fields) = combined_data.fields {
        fields
            .into_iter()
            .map(|field| {
                (
                    field.name,
                    field.body,
                    field.is_inline.parse::<bool>().unwrap_or(false),
                )
            })
            .collect()
    } else {
        vec![]
    };

    let final_embed = CreateEmbed::default()
        .title(data1.title)
        .description(data1.description)
        .footer(CreateEmbedFooter::new("").text(data1.footer.unwrap_or_else(|| "".to_string())))
        .author(CreateEmbedAuthor::new("").name(data1.author.unwrap_or_else(|| "".to_string())))
        .url(data2.url.unwrap_or_else(|| "".to_string()))
        .color(
            data2
                .color
                .unwrap_or_else(|| "".to_string())
                .parse::<u32>()
                .unwrap_or(0),
        )
        .image(data2.image.unwrap_or_else(|| "".to_string()))
        .thumbnail(data2.thumbnail.unwrap_or_else(|| "".to_string()))
        .fields(embed_fields);

    serenity::ChannelId::new(channel.id().get())
        .send_message(ctx.http(), CreateMessage::new().embed(final_embed))
        .await?;

    Ok(())
}
