use crate::EventHandler;
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

#[poise::async_trait]
impl serenity::EventHandler for EventHandler {
    async fn ready(&self, ctx: serenity::Context, data_about_bot: serenity::Ready) {
        let framework = self.framework.get();
        if let Some(self.framework.get()) = self.framework {
            
        }
        log::info!("{} is connected", data_about_bot.user.name);
    }
}