use poise::serenity_prelude::{self as serenity, Mention};
use crate::{Context, Error, Data};

pub enum MusicErrors {
    NoUserChannel,
}
#[poise::command(slash_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song"] user: String, // Here description is text attached to the command arguement description
) -> Result<(), Error> {
    let data = ctx.data();
    let guild_id = if let Some(guild) = ctx.guild_id() {guild} else {ctx.send(|r| r.content("You must be in a server to run this command")).await.unwrap(); return Ok(());};
    let channel_id = if let Some(channel_id) = ctx.guild().unwrap().voice_states.get(&ctx.author().id).and_then(|vc| vc.channel_id) {channel_id} else {ctx.send(|r| r.content("You must be in a server to run this command")).await.unwrap(); return Ok(());};
    //let sb = songbird::get(ctx.serenity_context()).await.expect("Cant find songbird instance from context");

    let result = data.songbird.join(guild_id, channel_id).await.0;
    result.lock().await.deafen(true).await.unwrap();
    Ok(())
}