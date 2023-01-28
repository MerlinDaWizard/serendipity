use crate::{Context, Error};
use crate::helpers::*;
use songbird::tracks::PlayMode::{Pause, Play};

#[poise::command(
    slash_command,
    guild_only,
    check = "crate::checks::same_channel",
)]
pub async fn pause(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let _data = ctx.data();
    let sb = songbird::get(ctx.discord()).await.expect("No songbird initialised").clone();

    match sb.get(ctx.guild_id().unwrap()) {
        Some(c) => {
            let call = c.lock().await;
            if call.queue().is_empty() {

            }
            match call.queue().current() {
                None => {
                    ctx.send(create_information_warning("There is nothing to pause", true).await).await?;
                    return Ok(());
                },
                Some(s) => {
                    if s.get_info().await?.playing == Pause {
                        ctx.send(create_information_warning("The current track is already paused", true).await).await?;
                        return Ok(())
                    }

                    call.queue().pause()?;
                    ctx.send(create_clear_embed("**⏸ | Paused!**").await).await?;
                }
            }
        },
        None => {
            ctx.send(create_information_warning("There is nothing to pause", true).await).await?;
        }
    }
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only,
    check = "crate::checks::same_channel",
)]
pub async fn resume(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let _data = ctx.data();
    let sb = songbird::get(ctx.discord()).await.expect("No songbird initialised").clone();

    match sb.get(ctx.guild_id().unwrap()) {
        Some(c) => {
            let call = c.lock().await;
            if call.queue().is_empty() {
                ctx.send(create_information_warning("There is nothing to resume", true).await).await?;
                return Ok(())
            }
            match call.queue().current() {
                None => {
                    ctx.send(create_information_warning("There is nothing to resume", true).await).await?;
                    return Ok(());
                },
                Some(s) => {
                    if s.get_info().await?.playing == Play {
                        ctx.send(create_information_warning("The current track is already playing", true).await).await?;
                        return Ok(())
                    }

                    call.queue().resume()?;
                    ctx.send(create_clear_embed("**⏯ | Resumed!**").await).await?;
                }
            }
        },
        None => {
            ctx.send(create_information_warning("There is nothing to resume", true).await).await?;
        }
    }
    Ok(())
}