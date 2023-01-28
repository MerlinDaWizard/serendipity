use poise::CreateReply;
use poise::serenity_prelude::{MessageBuilder, EmbedMessageBuilding, CreateEmbed, CreateEmbedAuthor};
use poise::serenity_prelude::{self as serenity};

use songbird::input::AuxMetadata;
use songbird::input::Compose;
use songbird::input::YoutubeDl;
use is_url::is_url;
use crate::time::DurationFormatter;
use crate::{Context, Error};
use crate::helpers::*;


pub struct AuxMetadataHolder;
impl songbird::typemap::TypeMapKey for AuxMetadataHolder {
    type Value = AuxMetadata;
}

pub struct Requestor;
impl songbird::typemap::TypeMapKey for Requestor {
    type Value = serenity::User;
}

#[poise::command(
    slash_command,
    guild_only,
    check = "crate::checks::bot_join_user",
)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song"] song: String, // Here description is text attached to the command arguement description
) -> Result<(), Error> {
    let _data = ctx.data();
    let guild_id = if let Some(guild) = ctx.guild_id() {guild} else {ctx.send(create_generic_error("You must be in a guild to use this command").await).await?; return Ok(());};
    let sb = songbird::get(ctx.discord()).await.expect("No songbird initialised").clone();

    let handler_lock = sb.get(guild_id).unwrap();
    let mut handler = handler_lock.lock().await;
    println!("P1");
    let reply_handle = ctx.send(CreateReply::new()
        .embed(CreateEmbed::new()
            .colour(INFO_EMBED_COLOUR)
            .description(":mag_right: **Searching...**")
    )).await?;
    println!("P2");
    let mut src = match is_url(&song) {
        true => YoutubeDl::new_ytdl_like("yt-dlp", reqwest::Client::new(), song),
        false => YoutubeDl::new_ytdl_like("yt-dlp", reqwest::Client::new(), format!("ytsearch:{}",song)),
    };
    
    println!("{:?}", src);
    let meta = src.aux_metadata().await;
    let track = handler.enqueue_input(src.into()).await;
    let mut typemap = track.typemap().write().await;
    typemap.insert::<Requestor>(ctx.author().clone());
    match meta {
        Ok(m) => {
            let thumbnail = &m.thumbnail;
            let title = &m.title;
            let source_url = &m.source_url;
            let requestor = ctx.author();
            let duration = &m.duration;

            reply_handle.edit(ctx, CreateReply::new()
                .embed(|| -> CreateEmbed {
                    let mut e = CreateEmbed::new();
                    e = e.colour(INFO_EMBED_COLOUR)
                    .author(CreateEmbedAuthor::new("Added to queue")
                        .icon_url(crate::config::ICON_URL)
                    )
                    .field("Added by", requestor.to_string(), true);
                    if let Some(duration) = duration {
                        e = e.field("Duration", format!("`{}`",DurationFormatter::new(duration).format_short()), true);
                        
                    }

                    if let Some(url) = source_url {
                        e = e.url(url);

                    }

                    if let Some(title) = title {
                        match source_url {
                            Some(u) => {
                                e = e.description(MessageBuilder::new().push_named_link_safe(title, u).build());
                            },
                            None => {
                                e = e.description(MessageBuilder::new().push_safe(title).build());
                            }
                        }
                    }

                    if let Some(url) = thumbnail {
                        e = e.thumbnail(url);
                    };
                    
                    return e;
            }())).await?;
            typemap.insert::<AuxMetadataHolder>(m);
        },
        Err(e) => {
            println!("Couldnt find metadata");
            println!("{:?}",e);
        }

    }
    return Ok(());
}