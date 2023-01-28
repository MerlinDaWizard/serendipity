
use poise::CreateReply;
use poise::serenity_prelude::{ChannelId, CreateEmbed};

use crate::{Context, Error};
use crate::helpers::*;
use crate::helpers;

pub async fn same_channel(ctx: Context<'_>) -> Result<bool, Error> {
    let guild = ctx.guild().unwrap();
    let bot_voice_opt = guild.voice_states.get(&ctx.data().bot_user_id).and_then(|vc| vc.channel_id);
    let user_channel = match guild.voice_states.get(&ctx.author().id).and_then(|vc| vc.channel_id) {
        Some(c) => c,
        None => {
            ctx.send(create_generic_error("You must be in the same voice channel as the bot to use this command").await).await?;
            return Ok(false);
        }
    };
    match bot_voice_opt {
        Some(bot_voice) => {
            match bot_voice.channel_id {
                Some(bot_channel) => {
                    if bot_channel == user_channel {
                        return Ok(true);
                    } else {
                        {ctx.send(create_generic_error("You must be in the same channel as the bot to use this command").await).await?;}
                        return Ok(false)
                    }
                },
                None => {
                    {ctx.send(create_generic_error("The bot must be in a channel to use this command").await).await?;}
                },
            }
        },
        None => {
            {ctx.send(create_generic_error("The bot must be in a channel to use this command").await).await?;}
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
                    ctx.send(create_generic_error("The bot must be in a channel to use this command").await).await?;
                    Ok(false)

                }
            }
        },
        None => {
            ctx.send(create_generic_error("The bot must be in a channel to use this command").await).await?;
            Ok(false)
        }
    }
}

pub async fn bot_join_user2(ctx: Context<'_>) -> Result<bool, Error> {
    
    // let bot_voice_opt = {
    //     let g = ctx.guild().unwrap();
    //     g.voice_states.get(&ctx.data().bot_user_id)
    // };
    let guild_id = ctx.guild_id().unwrap();
    let guild = ctx.discord().cache.guild(guild_id).unwrap().clone();
    // let bot_voice_opt = guild.voice_states.get(&ctx.data().bot_user_id);
    
    let user_channel = match guild.voice_states.get(&ctx.author().id).and_then(|vc| vc.channel_id) {
    //let user_channel = match Some(ChannelId::new(5)) {
        Some(c) => c,
        None => {
            //ctx.send(create_generic_error("You must be in a voice channel to use this command").await?;
            ctx.send(CreateReply::new()
            .embed(CreateEmbed::new()
                .colour(ERROR_COLOUR)
                .description(format!("❌ | **You must be in a voice channel to use this command**"))
            )).await?;
            return Ok(false);
        }
    };

    // // let bot_vs = match bot_voice_opt {
    // //     Some(vs) => vs,
    // //     None => {
    // //         // Bot not in any voice channel, should join users
    // //         ctx.data().songbird.join(guild_id, user_channel).await?;
    // //         return Ok(true);
    // //     },
    // // };

    // // let bot_channel = match bot_vs.channel_id {
    // //     Some(c) => c,
    // //     None => {
    // //         // Not sure how we are in a voice state but without a channel, cover for it anyways oc
    // //         ctx.data().songbird.join(guild_id, user_channel).await?;
    // //         return Ok(true);
    // //     },
    // // };

    // // if bot_channel != user_channel {
    // //     ctx.data().songbird.join(guild_id, user_channel).await?;
    // // }
    return Ok(true);
}

pub async fn bot_join_user(ctx: Context<'_>) -> Result<bool, Error> {
    let guild = ctx.guild().unwrap();
    let user_channel = match helpers::get_user_vc(&ctx) {
        Some(c) => c,
        None => {
            ctx.send(create_generic_error("You must be in a voice channel to use this command").await).await?;
            return Ok(false);
        }
    };

    let bot_voice_opt = guild.voice_states.get(&ctx.data().bot_user_id);
    let bot_vs = match bot_voice_opt {
        Some(vs) => vs,
        None => {
            // Bot not in any voice channel, should join users
            ctx.data().songbird.join(guild.id, user_channel).await?;
            return Ok(true);
        },
    };

    let bot_channel = match bot_vs.channel_id {
        Some(c) => c,
        None => {
            // Not sure how we are in a voice state but without a channel, cover for it anyways oc
            ctx.data().songbird.join(guild.id, user_channel).await?;
            return Ok(true);
        },
    };

    if bot_channel != user_channel {
        ctx.data().songbird.join(guild.id, user_channel).await?;
    }
    return Ok(true);
}