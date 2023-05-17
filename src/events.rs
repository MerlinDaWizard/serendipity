use std::{sync::Arc, time::Duration};

use crate::{config::DisconnectOptions, Data, Error, Timeouts};
use dashmap::DashMap;
use poise::serenity_prelude::FullEvent;
use serenity::{
    all::{ChannelId, GuildId},
    prelude::{CacheHttp, Context as SerenityContext},
};
use songbird::Songbird;
use tokio::time::{Instant, MissedTickBehavior};

pub async fn listener(
    //	ctx: &serenity::Context,
    event: &FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        #[rustfmt::skip]
        FullEvent::Ready { ctx, data_about_bot } => {
            log::info!("{} is connected!", data_about_bot.user.name);
            tokio::spawn(timeout_checker(ctx.clone(), framework.user_data().await.timeouts.clone()));

        }
        FullEvent::VoiceStateUpdate { ctx, old: _, new } => {
            let guild_id = new.guild_id.unwrap();
            if (data.bot_user_id == new.user_id) && new.channel_id.is_none() {
                let sb = songbird::get(ctx)
                    .await
                    .expect("No songbird initialised")
                    .clone();

                match sb.get(guild_id) {
                    Some(c) => {
                        let mut call = c.lock().await;
                        call.queue().stop();
                        call.leave().await?;
                    }
                    None => {
                        info!("Diconnected without call (Possibly due to restart)");
                    }
                }
            } else {
                if let Some(sb) = songbird::get(ctx).await {
                    if let Some(call) = sb.get(guild_id) {
                        let call = call.lock().await;
                        if let Some(bot_channel) = call.current_channel() {
                            let voice_goers = get_voice_goers(ctx, bot_channel.0.into()).await;

                            if voice_goers == 0 {
                                lonely_disconnect(
                                    data.config.voice_settings.on_lonely,
                                    guild_id,
                                    ctx,
                                    framework.user_data().await.timeouts.clone(),
                                    sb.clone(),
                                )
                                .await?;
                            } else {
                                if let Some(mut times) =
                                    framework.user_data().await.timeouts.get_mut(&guild_id)
                                {
                                    debug!("Clearing lonely timer. [{}]", guild_id.0);
                                    times.lonely_leavetime = None;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    //println!("{:?}",event);
    Ok(())
}

pub async fn lonely_disconnect(
    action_type: DisconnectOptions,
    guild: GuildId,
    _ctx: &poise::serenity_prelude::Context,
    timeouts: Arc<DashMap<GuildId, Timeouts>>,
    sb: Arc<Songbird>,
) -> Result<(), Error> {
    match action_type {
        DisconnectOptions::Timeout(s) => {
            debug!("Setting lonely timer. [{}]", guild.0);
            match timeouts.get_mut(&guild) {
                Some(mut times) => {
                    times.lonely_leavetime = Some(Instant::now() + Duration::from_secs(s as u64));
                }
                None => {
                    timeouts.insert(
                        guild,
                        Timeouts {
                            lonely_leavetime: Some(Instant::now() + Duration::from_secs(s as u64)),
                            idle_leavetime: None,
                        },
                    );
                }
            }
            // tokio::spawn(timeout(s, guild, ctx.clone(), ctx.data.clone()));
        }
        DisconnectOptions::Instant => {
            info!("Left call in {} due to lonely.", guild.0);
            sb.leave(guild).await?;
        }
        DisconnectOptions::Off => (),
    };
    Ok(())
}

// pub async fn timeout(
//     seconds: usize,
//     guild: GuildId,
//     cache_http: impl CacheHttp,
//     data: Arc<RwLock<TypeMap>>,
// ) {
//     info!("Started timeout [{}s]", seconds);
//     tokio::time::sleep(Duration::from_secs(seconds as u64)).await;
//     info!("Left call in {} due to lonely timeout.", guild.0);
//     let data = data.read().await;

//     let sb = data.get::<SongbirdKey>().cloned().unwrap();
//     let call_mutex = sb.get(guild).unwrap();
//     let mut call = call_mutex.lock().await;

//     let vc = call.current_channel().unwrap();

//     let voice_goers = get_voice_goers(cache_http, vc.0.into()).await;
//     if voice_goers > 0 {
//         info!("Ignoring timeout, no longer lonely.");
//     } else {
//         call.leave().await.unwrap();
//     }
// }

pub async fn get_voice_goers(cache_http: impl CacheHttp, channel: ChannelId) -> usize {
    let voice_goers = cache_http
        .cache()
        .unwrap()
        .guild_channel(channel)
        .unwrap()
        .members(cache_http.cache().unwrap())
        .unwrap()
        .iter()
        .filter(|member| !member.user.bot)
        .count();

    voice_goers
}

// Note: I could make this create its own tasks which would further act in parallel but nah. [Todo:]
async fn timeout_checker(ctx: SerenityContext, times: Arc<DashMap<GuildId, Timeouts>>) {
    let mut interval = tokio::time::interval(Duration::from_millis(500));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        interval.tick().await;
        trace!("Dc tick");

        for mut multi_ref in times.iter_mut() {
            let (guild, times) = multi_ref.pair_mut();

            compare_time(&ctx, guild, &mut times.idle_leavetime).await;
            compare_time(&ctx, guild, &mut times.lonely_leavetime).await;
        }
    }
}

async fn compare_time(ctx: &SerenityContext, guild: &GuildId, time: &mut Option<Instant>) {
    match time {
        Some(instant) => {
            if *instant < tokio::time::Instant::now() {
                *time = None;
                let sb = songbird::get(ctx).await.unwrap();
                match sb.leave(guild.0).await {
                    Ok(_) => info!("Left channel due to lonely. [{}]", guild.0),
                    Err(e) => info!("Could not leave channel due to: {}", e),
                }
            }
        }
        None => (),
    }
}
