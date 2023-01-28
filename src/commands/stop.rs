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
    let sb = songbird::get(ctx.discord()).await.expect("No songbird initialised").clone();

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
    ctx.send(create_clear_embed("**:wave: | Bye Bye!**").await).await?;
    Ok(())
}