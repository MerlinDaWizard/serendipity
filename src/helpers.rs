

use poise::serenity_prelude::{Colour, ChannelId};
use crate::{Context, Error};

pub const ERROR_COLOUR: Colour = Colour::from_rgb(237,66,69);
pub const CLEAR_EMBED_COLOUR: Colour = Colour::from_rgb(54,57,63);
pub const INFO_EMBED_COLOUR: Colour = CLEAR_EMBED_COLOUR;

pub async fn generic_error<D: std::fmt::Display>(ctx: &Context<'_>, msg: D) -> Result<(), Error> {

    ctx.send(|r|
        r.embed(|e|
            e.colour(ERROR_COLOUR)
            .description(format!("❌ | **{msg}**"))
    )).await?;
    Ok(())
}

pub async fn send_information_warning<D: std::fmt::Display>(ctx: &Context<'_>, msg: D, ephemeral: bool) -> Result<(), Error> {

    ctx.send(|r|
        r.embed(|e|
            e.colour(ERROR_COLOUR)
            .description(msg)
    ).ephemeral(ephemeral)).await?;
    Ok(())
}

pub async fn send_clear_embed<D: std::fmt::Display>(ctx: &Context<'_>, msg: D) -> Result<(), Error> {

    ctx.send(|r|
        r.embed(|e|
            e.colour(INFO_EMBED_COLOUR)
            .description(msg)
    )).await?;
    Ok(())
}


/// Only should be used when already checked if its in a guild
pub fn get_user_vc(ctx: &Context<'_>) -> Option<ChannelId> {
    ctx.guild().unwrap().voice_states.get(&ctx.author().id).and_then(|vc| vc.channel_id)
}