
use crate::{Context, Error};
use crate::helpers::*;
use crate::helpers;

pub async fn same_channel(ctx: Context<'_>) -> Result<bool, Error> {
    let guild = ctx.guild().unwrap();
    let user_channel = match helpers::get_user_vc(&ctx) {
        Some(c) => c,
        None => {
            generic_error(&ctx, "You must be in the same voice channel as the bot to use this command").await?;
            return Ok(false);
        }
    };
    let bot_voice_opt = guild.voice_states.get(&ctx.data().bot_user_id);

    match bot_voice_opt {
        Some(bot_voice) => {
            match bot_voice.channel_id {
                Some(bot_channel) => {
                    if bot_channel == user_channel {
                        return Ok(true);
                    } else {
                        generic_error(&ctx, "You must be in the same channel as the bot to use this command").await?;
                        return Ok(false)
                    }
                },
                None => {
                    generic_error(&ctx, "The bot must be in a channel to use this command").await?;
                },
            }
        },
        None => {
            generic_error(&ctx, "The bot must be in a channel to use this command").await?;
            return Ok(false);
        }
    };
    Ok(true)
}

pub async fn bot_in_vc(ctx: Context<'_>) -> Result<bool, Error> {
    let guild = ctx.guild().unwrap();
    let bot_voice_opt = guild.voice_states.get(&ctx.data().bot_user_id);
    match bot_voice_opt {
        Some(bot_voice) => {
            match bot_voice.channel_id {
                Some(_) => {
                    Ok(true)
                },
                None => {
                    generic_error(&ctx, "The bot must be in a channel to use this command").await?;
                    Ok(false)

                }
            }
        },
        None => {
            generic_error(&ctx, "The bot must be in a channel to use this command2").await?;
            Ok(false)
        }
    }
}
