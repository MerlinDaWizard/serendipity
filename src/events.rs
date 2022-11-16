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
        }
        _ => {}
    }
    //println!("{:?}",event);
    Ok(())
}
