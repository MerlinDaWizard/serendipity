use crate::{Context, Error};
use crate::helpers::*;
use std::time::Duration;
use humantime::format_duration;
use poise::serenity_prelude::MessageBuilder;

#[poise::command(
    slash_command,
    guild_only,
    check = "crate::checks::same_channel",
)]
pub async fn seek(
    ctx: Context<'_>,
    #[description = "Sit"] 
    duration: String,
) -> Result<(), Error> {
    let _data = ctx.data();
    let guild_id = ctx.guild_id().unwrap();
    let sb = songbird::get(ctx.discord()).await.expect("No songbird initialised").clone();
    let position = match humantime::parse_duration(&duration) {
        Ok(d) => d,
        Err(e) => {
            ctx.send(create_information_warning(format!("Error while parsing seek position: {e}"), true).await).await?;
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
    ctx.defer().await?;
    match current.seek(position).result() {
        Ok(d) => {
            // Kinda weird fix to round floor it down.
            let time_display = format_duration(d - Duration::from_millis(d.subsec_millis() as u64)).to_string();
            ctx.send(create_clear_embed(MessageBuilder::new().push("⏩ | Current song has been set to ").push_bold(time_display).build()).await).await?;
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