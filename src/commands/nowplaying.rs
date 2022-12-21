

use crate::{Context, Error};
use crate::helpers::*;
use crate::time::DurationFormatter;

#[poise::command(
    slash_command,
    guild_only,
    check = "crate::checks::bot_in_vc",
)]
pub async fn nowplaying(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let _data = ctx.data();
    let sb = songbird::get(ctx.serenity_context()).await.expect("No songbird initialised").clone();

    match sb.get(ctx.guild_id().unwrap()) {
        Some(c) => {
            let call = c.lock().await;
            let q = call.queue();
            match q.current() {
                Some(sh) => {
                    let typemap = sh.typemap().read().await;
                    let meta = typemap.get::<crate::commands::play::AuxMetadataHolder>().expect("Expected metadata");
                    let requestor = typemap.get::<crate::commands::play::Requestor>().expect("Requestor not found");
                    let unknown_text = "unknown".to_string();
                    let title = meta.title.as_ref().unwrap_or(&unknown_text);
                    let video_icon_url = &meta.thumbnail;
                    let source_url = &meta.source_url;
                    let position = sh.get_info().await.unwrap().position;
                    let song_length = meta.duration;
                    // if let Some(icon_url) = icon_url {
                    //     a.icon_url(icon_url)
                    // };
                    // if let Some(sour)
                    ctx.send(|r|
                        r.embed(|e| {
                            e.colour(crate::helpers::INFO_EMBED_COLOUR)
                            .author(|a|
                                a.name("Now Playing")
                                .icon_url(crate::config::ICON_URL)
                            )
                            .field("Requested by", requestor.to_string(), true);
                            match song_length {
                                Some(l) => {
                                    e.field("Duration", format!("`{} / {}`", DurationFormatter::new(position).format_short(), DurationFormatter::new(l).format_short()), true);
                                },
                                None => {
                                    e.field("Position", format!("`{}`", DurationFormatter::new(position).format_short()), true);
                                }
                            };
                            //.field("Requested by", requestor.to_string(), true)

                            if let Some(url) = video_icon_url {
                                e.thumbnail(url);
                            };

                            match source_url {
                                Some(url) => {
                                    e.description(format!("[{title}]({url})"));
                                },
                                None => {
                                    e.description(title);
                                }
                            };

                            e
                        }
                    )).await?;
                    println!("{:?}", meta);
                    sh.get_info().await?;
                },
                None => {
                    send_information_warning(&ctx, "There's nothing playing.", true).await?
                }
            }
        },
        None => {
            send_information_warning(&ctx, "There's nothing playing.", true).await?;
        }
    }
    Ok(())
}