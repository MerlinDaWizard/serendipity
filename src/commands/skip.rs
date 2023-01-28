

use crate::{Context, Error};
use crate::helpers::*;

#[poise::command(
    slash_command,
    guild_only,
    check = "crate::checks::same_channel",
)]
pub async fn skip(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let _data = ctx.data();
    let sb = songbird::get(ctx.discord()).await.expect("No songbird initialised").clone();

    match sb.get(ctx.guild_id().unwrap()) {
        Some(c) => {
            let call = c.lock().await;
            if call.queue().len() == 0 {
                ctx.send(create_information_warning("There is nothing to skip", true).await).await?;
                return Ok(())
            }
            call.queue().skip()?;
            ctx.send(create_clear_embed("**✅ | Skipped!**").await).await?;
        },
        None => {
            ctx.send(create_generic_error("**There is nothing to skip**").await).await?;
        }
    }
    Ok(())
}