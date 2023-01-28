use poise::{serenity_prelude::{Colour, ChannelId, CreateEmbed}, CreateReply};
use crate::{Context};
pub const ERROR_COLOUR: Colour = Colour::from_rgb(237,66,69);
pub const CLEAR_EMBED_COLOUR: Colour = Colour::from_rgb(54,57,63);
pub const INFO_EMBED_COLOUR: Colour = CLEAR_EMBED_COLOUR;

/// Only should be used when already checked if its in a guild
pub fn get_user_vc(ctx: &Context<'_>) -> Option<ChannelId> {
    ctx.guild().unwrap().voice_states.get(&ctx.author().id).and_then(|vc| vc.channel_id)
}

pub async fn create_generic_error<D: std::fmt::Display>(msg: D) -> CreateReply {
    let msg = msg.to_string();
    
    CreateReply::new()
        .embed(CreateEmbed::new()
            .colour(ERROR_COLOUR)
            .description(msg)
        )
}

pub async fn create_information_warning<D: std::fmt::Display>(msg: D, ephemeral: bool) -> CreateReply {
    let msg = msg.to_string();
    
    CreateReply::new()
        .embed(CreateEmbed::new()
            .colour(ERROR_COLOUR)
            .description(msg)
    ).ephemeral(ephemeral)
}

pub async fn create_clear_embed<D: std::fmt::Display>(msg: D) -> CreateReply {
    let msg = msg.to_string();
    
    CreateReply::new()
        .embed(CreateEmbed::new()
            .colour(CLEAR_EMBED_COLOUR)
            .description(msg)
        )
}

pub async fn create_simple_embed<D: std::fmt::Display>(colour: Colour, msg: D) -> CreateReply {
    let msg = msg.to_string();
    
    CreateReply::new()
        .embed(CreateEmbed::new()
            .colour(colour)
            .description(msg)
        )
}