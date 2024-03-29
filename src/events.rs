
use poise::serenity_prelude::FullEvent;
use crate::{Error, Data};

pub async fn listener(
//	ctx: &serenity::Context,
	event: &FullEvent,
	_framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { ctx: _, data_about_bot } => {
            log::info!("{} is connected!", data_about_bot.user.name);
        },
        FullEvent::VoiceStateUpdate { ctx, old: _, new } => {
            if (data.bot_user_id == new.user_id) && new.channel_id.is_none() {
                let sb = songbird::get(ctx).await.expect("No songbird initialised").clone();
                match sb.get(new.guild_id.unwrap()) {
                    Some(c) => {
                        let mut call = c.lock().await;
                        call.queue().stop();
                        call.leave().await?;
                    },
                    None => {
                        println!("No call on dc");
                    }
                }
            }
        }
        _ => {}
    }
    //println!("{:?}",event);
    Ok(())
}
