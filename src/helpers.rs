use std::sync::Arc;

use poise::CreateReply;
use serenity::{builder::CreateEmbed, model::Colour};
use songbird::Call;
use tokio::sync::Mutex;

use crate::{errors::ParsimonyErrors, Context, Error};

pub const ERROR_COLOUR: Colour = Colour::from_rgb(237, 66, 69);
pub const CLEAR_EMBED_COLOUR: Colour = Colour::from_rgb(54, 57, 63);
pub const INFO_EMBED_COLOUR: Colour = CLEAR_EMBED_COLOUR;
pub const ICON_URL: &str = "https://cdn.darrennathanael.com/icons/spinning_disk.gif";

pub fn create_information_warning<D: std::fmt::Display>(msg: D, ephemeral: bool) -> CreateReply {
    let msg = msg.to_string();

    CreateReply::new()
        .embed(CreateEmbed::new().colour(ERROR_COLOUR).description(msg))
        .ephemeral(ephemeral)
}

pub async fn create_clear_embed<D: std::fmt::Display>(msg: D) -> CreateReply {
    let msg: String = msg.to_string();

    CreateReply::new().embed(
        CreateEmbed::new()
            .colour(CLEAR_EMBED_COLOUR)
            .description(msg),
    )
}

pub async fn join_author_vc(ctx: &Context<'_>) -> Result<Arc<Mutex<Call>>, Error> {
    let user_channel = {
        let guild = ctx.guild().unwrap();
        guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vc| vc.channel_id)
    };

    if let Some(user_channel) = user_channel {
        let sb = songbird::get(ctx.discord()).await.unwrap();
        let call = sb
            .join(ctx.guild_id().unwrap(), user_channel)
            .await
            .unwrap();
        Ok(call)
    } else {
        Err(Box::new(ParsimonyErrors::AuthorNoVC))
    }
}

pub fn sanitise_text(text: &str) -> String {
    let mut buf = String::with_capacity(2 * text.len());
    for char in text.chars() {
        let add = match char {
            '\\' => "\\\\",
            '*' => r"\*",
            '~' => r"\~",
            '_' => r"\_",
            '[' => r"\[",
            ']' => r"\]",
            '|' => r"\|",
            '`' => r"\`",
            '<' => r"\<",
            '>' => r"\>",
            _ => {
                buf.push(char);
                continue;
            }
        };

        buf.push_str(add);
    }

    buf.shrink_to_fit();
    buf
}

pub fn stop_main_pings(mut text: String) -> String {
    text = text.replace("@here", "@\u{200B}here");
    text = text.replace("@everyone", "@\u{200B}everyone");
    text
}
