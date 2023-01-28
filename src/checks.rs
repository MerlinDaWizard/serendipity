



use crate::{Context, Error};
use crate::helpers::*;


pub async fn same_channel(ctx: Context<'_>) -> Result<bool, Error> {
    // CacheRef cant be passed across await so we do this :D
    let (bot_channel, user_channel) = {
        let guild = ctx.guild().unwrap();
        let bot_channel = guild.voice_states.get(&ctx.data().bot_user_id).and_then(|vc| vc.channel_id);
        let user_channel = guild.voice_states.get(&ctx.author().id).and_then(|vc| vc.channel_id);
        (bot_channel, user_channel)
    };

    if bot_channel != user_channel {
        ctx.send(create_generic_error("You must be in the same voice channel as the bot to use this command").await).await?;
        return Ok(false);
    }
    Ok(true)
}

pub async fn bot_in_vc(ctx: Context<'_>) -> Result<bool, Error> {
    let bot_channel = {
        let guild = ctx.guild().unwrap();
        let bot_channel = guild.voice_states.get(&ctx.data().bot_user_id).and_then(|vc| vc.channel_id);
        bot_channel
    };

    match bot_channel {
        Some(_) => Ok(true),
        None => {
            ctx.send(create_generic_error("The bot must be in a channel to use this command").await).await?;
            Ok(false)
        },
    }
}

pub async fn bot_join_user(ctx: Context<'_>) -> Result<bool, Error> {
    let (bot_channel, user_channel) = {
        let guild = ctx.guild().unwrap();
        let bot_channel = guild.voice_states.get(&ctx.data().bot_user_id).and_then(|vc| vc.channel_id);
        let user_channel = guild.voice_states.get(&ctx.author().id).and_then(|vc| vc.channel_id);
        (bot_channel, user_channel)
    };

    let user_channel = match user_channel {
        Some(c) => c,
        None => {

            return Ok(false);
        }
    };

    // if bot_channel.is_some() && bot_channel.unwrap() == user_channel {
    //     return Ok(true);
    // }

    let manager = songbird::get(ctx.discord()).await.expect("Songbird not initialised").clone();
    manager.join(ctx.guild_id().unwrap(), user_channel).await?;
    Ok(true)
}