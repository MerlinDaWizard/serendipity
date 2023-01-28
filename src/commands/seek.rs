use crate::{Context, Error};
use crate::helpers::*;
use crate::time::DurationFormatter;

use ms_converter::ms_into_time;
use poise::serenity_prelude::MessageBuilder;

#[poise::command(
    slash_command,
    guild_only,
    check = "crate::checks::same_channel",
)]
pub async fn seek(
    ctx: Context<'_>,
    duration: String,
) -> Result<(), Error> {
    let _data = ctx.data();
    let guild_id = ctx.guild_id().unwrap();
    let sb = songbird::get(ctx.discord()).await.expect("No songbird initialised").clone();
    let position = match ms_into_time(duration) {
        Ok(d) => d,
        Err(e) => {
            ctx.send(create_information_warning(format!("Error while parsing seek position: {}", e), true).await).await?;
            return Ok(());
        }
    };

    let handler_lock = sb.get(guild_id).unwrap();
    let handler = handler_lock.lock().await;
    let current = match handler.queue().current() {
        Some(s) => s,
        None => {
            ctx.send(create_information_warning("There is nothing to seek", true).await).await?;
            return Ok(())
        }
    };
    
    // let typemap = current.typemap().read().await;
    // let meta = typemap.get::<crate::commands::play::AuxMetadataHolder>().expect("Expected metadata");
    println!("{:?}",current.get_info().await?);
    println!("BEFORE");
    match current.seek(position).result_async().await {
        Ok(d) => {
            ctx.send(create_clear_embed(MessageBuilder::new().push("⏩ | Current song has been set to ").push_bold(DurationFormatter::new(&d).format_short()).build()).await).await?;
            // match &meta.title {
            //     Some(t) => {
            //         println!("Yoo");
            //         dbg!(d);
            //         send_clear_embed(&ctx, MessageBuilder::new().push("⏩ | ").push_bold_safe(t).push(" has been set to ").push_bold(DurationFormatter::new(&d).format_short()).build()).await?;
            //     },
            //     None => {
            //         send_clear_embed(&ctx, MessageBuilder::new().push("⏩ | Current song has been set to ").push_bold(DurationFormatter::new(&d).format_short()).build()).await?;
            //     }
            // }
        },
        Err(e) => {
            dbg!(&e);
            ctx.send(create_information_warning(format!("Error while seeking: {e}"), true).await).await?;
        }
    }
    println!("AFTER");
    Ok(())
}