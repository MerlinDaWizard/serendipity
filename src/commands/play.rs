


use poise::serenity_prelude::{self as serenity, ChannelId};

use songbird::input::AuxMetadata;
use songbird::input::Compose;
use songbird::input::YoutubeDl;


use crate::{Context, Error, Data};
use crate::helpers::*;


pub struct AuxMetadataHolder;
impl songbird::typemap::TypeMapKey for AuxMetadataHolder {
    type Value = AuxMetadata;
}

pub struct Requestor;
impl songbird::typemap::TypeMapKey for Requestor {
    type Value = serenity::User;
}


#[poise::command(slash_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song"] song: String, // Here description is text attached to the command arguement description
) -> Result<(), Error> {
    let data = ctx.data();
    
    let guild_id = if let Some(guild) = ctx.guild_id() {guild} else {generic_error(&ctx, "You must be in a guild to use this command").await?; return Ok(());};
    let guild = ctx.guild().unwrap();
    let channel_id = if let Some(channel_id) = get_user_vc(&ctx) {
        channel_id
    } else {
        generic_error(&ctx, "You must be in a voice channel to use this command").await?; return Ok(());
    };

    println!("Hello1!");
    let bot_in_vc = guild.voice_states.get(&data.bot_user_id);
    match bot_in_vc {
        Some(vc) => {
            if vc.channel_id.unwrap() != channel_id {
                generic_error(&ctx, "You must be in the same voice channel as me to use this command!").await?;
                return Ok(());
            }
        },
        None => {
            join_vc(data, guild_id, channel_id).await?;
        }
    }
    //let sb = songbird::get(ctx.serenity_context()).await.expect("Cant find songbird instance from context");
    //let sb = data.songbird.clone();
    println!("Hello1.5!");
    let sb = songbird::get(ctx.serenity_context()).await.expect("No songbird initialised").clone();
    println!("Hello2!");
    // let source = match  {
    //     Ok(s) => s,
    //     Err(e) => {
    //         log::error!("Playback error: {:?}", e);
    //         generic_error(&ctx, format!("Playback error: {:?}", e)).await?;
    //         return Ok(())
    //     },
    // };
    if let Some(handler_lock) = sb.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        //let mut src = File::new("bluesky.mp3");
        let mut src = YoutubeDl::new_ytdl_like("yt-dlp", reqwest::Client::builder().build().unwrap(), song);
        let meta = src.aux_metadata().await;
        //let track = handler.play_input(src.into());
        let track = handler.enqueue_input(src.into()).await;

        let mut typemap = track.typemap().write().await;
        typemap.insert::<Requestor>(ctx.author().clone());
        match meta {
            Ok(m) => {
                typemap.insert::<AuxMetadataHolder>(m);
                
            },
            Err(e) => {
                println!("Couldnt find metadata, generating");
                println!("{:?}",e);
            }

        }
        //track.typemap(AuxMetadata);

        //track.play()?;
        println!("Hello5!");
    } else {
        println!("Not in channel issue");
    }
    Ok(())
}

pub async fn join_vc(data: &Data, guild_id: poise::serenity_prelude::GuildId, channel_id: ChannelId) -> Result<(), Error> {
    let result = data.songbird.join(guild_id, channel_id).await;
    result.1?;
    result.0.lock().await.deafen(true).await?;
    Ok(())
}

// impl Into<songbird::id::ChannelId> for ChannelId {
//     fn into(self) -> songbird::id::ChannelId {
//         todo!()
//     }
// }