use poise::serenity_prelude::{self as serenity, Mention, ChannelId};
use songbird::tracks::TrackResult;
use crate::{Context, Error, Data};
use crate::helpers::*;

#[poise::command(
    slash_command,
    guild_only,
    check = "crate::checks::same_channel",
)]
pub async fn skip(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let data = ctx.data();
    let sb = songbird::get(ctx.serenity_context()).await.expect("No songbird initialised").clone();

    match sb.get(ctx.guild_id().unwrap()) {
        Some(c) => {
            let call = c.lock().await;
            if call.queue().len() <= 0 {
                send_information_warning(&ctx, "There is nothing to skip", true).await?;
                return Ok(())
            }
            call.queue().skip()?;
            send_clear_embed(&ctx, "**✅ | Skipped!**").await?;
        },
        None => {
            generic_error(&ctx, "**There is nothing to skip**").await?;
        }
    }
    return Ok(())
}