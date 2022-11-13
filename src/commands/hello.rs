use poise::serenity_prelude::{self as serenity, Mention};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn hello(
    ctx: Context<'_>, // <'_> means an anonomous lifetime
    #[description = "Selected user"] user: serenity::User, // Here description is text attached to the command arguement description
) -> Result<(), Error> { // Returns an empty tuple, meaning success (see ok below) or an error
    ctx.say(format!("{} says hello to {} :D", Mention::from(ctx.author().id), Mention::from(user.id))).await?;
    Ok(())
}