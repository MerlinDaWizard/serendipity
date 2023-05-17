use poise::CreateReply;
use serenity::{builder::CreateEmbed, model::Colour};

use crate::{Context, Error};

#[poise::command(slash_command, guild_only)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let (_bot_channel, user_channel) = {
        let guild = ctx.guild().unwrap();
        let bot_channel = guild
            .voice_states
            .get(&ctx.data().bot_user_id)
            .and_then(|vc| vc.channel_id);
        let user_channel = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vc| vc.channel_id);
        (bot_channel, user_channel)
    };

    let user_channel = match user_channel {
        Some(c) => c,
        None => {
            let reply = CreateReply::new().embed(
                CreateEmbed::new()
                    .colour(Colour::from_rgb(237, 66, 69))
                    .description("You are not in a voice channel"),
            );

            ctx.send(reply).await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird not initialised")
        .clone();
    manager.join(ctx.guild_id().unwrap(), user_channel).await?;

    let reply = CreateReply::new().embed(
        CreateEmbed::new()
            .colour(Colour::from_rgb(54, 57, 63))
            .description("Joined"),
    );

    ctx.send(reply).await?;
    Ok(())
}
