use songbird::tracks::{LoopState, TrackHandle};

use crate::{errors::ParsimonyErrors, helpers, Context, Error, GuildState};

use super::play::AuxMetadataHolder;

#[poise::command(slash_command, guild_only, rename = "loop")]
pub async fn loops(ctx: Context<'_>, loop_type: LoopType) -> Result<(), Error> {
    match loop_type {
        LoopType::Song => loop_song(&ctx).await?,
        LoopType::Queue => loop_queue(&ctx).await?,
    }

    Ok(())
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum LoopType {
    #[name = "Queue"]
    Queue,
    #[name = "Song"]
    Song,
}

async fn loop_song(ctx: &Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild_id().unwrap();
    let sb = &ctx.data().songbird;

    let call = sb.get(guild.0).ok_or(ParsimonyErrors::NoActiveCall)?;
    let call_lock = call.lock().await;

    let queue = call_lock.queue();
    let current = queue.current().ok_or(ParsimonyErrors::NothingQueued)?;

    let info = current
        .get_info()
        .await
        .map_err(|_| ParsimonyErrors::NothingQueued)?;

    match info.loops {
        LoopState::Infinite => {
            current.disable_loop()?;
            loop_song_message(ctx, current, false).await?;
        }
        LoopState::Finite(amount) => match amount {
            0 => {
                current.enable_loop()?;
                loop_song_message(ctx, current, true).await?;
            }
            _ => {
                todo!();
            }
        },
    }
    Ok(())
}

async fn loop_queue(ctx: &Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let guild_id = ctx.guild_id().unwrap();
    let guild_state = data.guild_states.get_mut(&ctx.guild_id().unwrap());
    match guild_state {
        Some(mut guild_state) => {
            guild_state.loop_queue = !guild_state.loop_queue;
            loop_queue_message(ctx, guild_state.loop_queue).await?;
        }
        None => {
            data.guild_states.insert(
                guild_id,
                GuildState {
                    loop_queue: true,
                    ..Default::default()
                },
            );
            loop_queue_message(ctx, true).await?;
        }
    }
    Ok(())
}

async fn loop_queue_message(ctx: &Context<'_>, loop_state: bool) -> Result<(), Error> {
    let reply = match loop_state {
        true => {
            helpers::create_clear_embed(":arrows_counterclockwise: | **Started** looping the queue")
        }
        false => helpers::create_clear_embed(":octagonal_sign: | **Stopped** looping the queue"),
    }
    .await;

    ctx.send(reply).await?;
    Ok(())
}

async fn loop_song_message(
    ctx: &Context<'_>,
    current: TrackHandle,
    loop_state: bool,
) -> Result<(), Error> {
    let typemap_lock = current.typemap().read().await;
    let meta = typemap_lock.get::<AuxMetadataHolder>();

    let title = match meta {
        Some(meta) => (&meta.title)
            .as_ref()
            .unwrap_or(&"Unknown".to_string())
            .clone(),
        None => "Unknown".to_string(),
    };

    let title = helpers::sanitise_text(helpers::stop_main_pings(title).as_str());
    let embd = match loop_state {
        true => {
            helpers::create_clear_embed(format!(
                ":arrows_counterclockwise: | **Started** looping *{}*",
                title
            ))
            .await
        }
        false => {
            helpers::create_clear_embed(format!(
                ":arrows_counterclockwise: | **Disabled** looping *{}*",
                title
            ))
            .await
        }
    };

    ctx.send(embd).await?;
    Ok(())
}
