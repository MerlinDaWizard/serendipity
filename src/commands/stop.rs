
use crate::{Context, Error};
use crate::helpers::*;

#[poise::command(
    slash_command,
    guild_only,
    check = "crate::checks::same_channel",
)]
pub async fn stop(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let _data = ctx.data();
    let sb = songbird::get(ctx.serenity_context()).await.expect("No songbird initialised").clone();

    match sb.get(ctx.guild_id().unwrap()) {
        Some(c) => {
            let mut call = c.lock().await;
            call.queue().stop();
            call.leave().await?;
        },
        None => {
            sb.leave(ctx.guild_id().unwrap()).await?;
        }
    }
    send_clear_embed(&ctx, "**:wave: | Bye Bye!**").await?;
    Ok(())
}