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
    let sb = songbird::get(ctx.serenity_context()).await.expect("No songbird initialised").clone();

    match sb.get(ctx.guild_id().unwrap()) {
        Some(c) => {
            let call = c.lock().await;
            if call.queue().len() == 0 {

            }
            match call.queue().current() {
                None => {
                    send_information_warning(&ctx, "There is nothing to pause", true).await?;
                    return Ok(());
                },
                Some(s) => {
                    if s.get_info().await?.playing == Pause {
                        send_information_warning(&ctx, "The current track is already paused", true).await?;
                        return Ok(())
                    }

                    call.queue().pause()?;
                    send_clear_embed(&ctx, "**⏸ | Paused!**").await?;
                }
            }
        },
        None => {
            send_information_warning(&ctx, "There is nothing to pause", true).await?;
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
    let sb = songbird::get(ctx.serenity_context()).await.expect("No songbird initialised").clone();

    match sb.get(ctx.guild_id().unwrap()) {
        Some(c) => {
            let call = c.lock().await;
            if call.queue().len() == 0 {
                send_information_warning(&ctx, "There is nothing to resume", true).await?;
                return Ok(())
            }
            match call.queue().current() {
                None => {
                    send_information_warning(&ctx, "There is nothing to resume", true).await?;
                    return Ok(());
                },
                Some(s) => {
                    if s.get_info().await?.playing == Play {
                        send_information_warning(&ctx, "The current track is already playing", true).await?;
                        return Ok(())
                    }

                    call.queue().resume()?;
                    send_clear_embed(&ctx, "**⏯ | Resumed!**").await?;
                }
            }
        },
        None => {
            send_information_warning(&ctx, "There is nothing to resume", true).await?;
        }
    }
    Ok(())
}