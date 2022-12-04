use poise::serenity_prelude::{self as serenity, Mention, GuildId, ChannelId};
use songbird::Call;
use crate::{Context, Error, Data};
use crate::helpers::*;
pub enum MusicErrors {
    NoUserChannel,
}

#[poise::command(slash_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song"] user: String, // Here description is text attached to the command arguement description
) -> Result<(), Error> {
    let data = ctx.data();
    
    let guild_id = if let Some(guild) = ctx.guild_id() {guild} else {generic_error(&ctx, "You must be in a guild to use this command").await?; return Ok(());};
    let guild = ctx.guild().unwrap();
    let channel_id = if let Some(channel_id) = get_user_vc(&ctx) {
        channel_id
    } else {
        generic_error(&ctx, "You must be in a voice channel to use this command").await?; return Ok(());
    };

    let bot_in_vc = guild.voice_states.get(&data.bot_user_id);
    match bot_in_vc {
        Some(vc) => {
            if vc.channel_id.unwrap() != channel_id {
                generic_error(&ctx, "You must be in the same voice channel as me to use this command!").await?
            }
        },
        None => {
            join_vc(data, guild_id, channel_id).await?;
        }
    }
    //let sb = songbird::get(ctx.serenity_context()).await.expect("Cant find songbird instance from context");
    Ok(())
}

pub async fn join_vc(data: &Data, guild_id: GuildId, channel_id: ChannelId) -> Result<(), Error> {
    let result = data.songbird.join(guild_id, channel_id).await.0;
    result.lock().await.deafen(true).await?;
    Ok(())

}