use poise::serenity_prelude::{self as serenity};
use crate::{Error, Data};

pub async fn listener(
	ctx: &serenity::Context,
	event: &poise::Event<'_>,
	framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            log::info!("{} is connected!", data_about_bot.user.name);
        },
        poise::Event::VoiceStateUpdate { old, new } => {
            println!("Got voice state update");
            println!("{:?}",new);
            // TODO
            if data.bot_user_id == new.user_id && new.deaf == false {
                if let Some(guild) = new.guild_id { // Not too sure how this could be false but /shrug
                    ctx.cache.member(guild, data.bot_user_id);
                } else {
                    println!("Damn this got triggered")
                }
            }
        }
        _ => {}
    }
    //println!("{:?}",event);
    Ok(())
}
