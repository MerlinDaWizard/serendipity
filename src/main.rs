#![allow(stable_features)]
mod commands;
mod events;
//use crate::events::listener;
use std::{env, sync::{Arc, atomic::AtomicBool}};

use dotenv::dotenv;
use poise::{serenity_prelude::{self as serenity, UserId, Ready, builder::*, ClientBuilder}, async_trait, Framework, event::EventWrapper};
//use songbird::serenity;
use songbird::serenity::SerenityInit;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    songbird: Arc<songbird::Songbird>,

    bot_start_time: std::time::Instant,
    bot_user_id: UserId,
    version: String,
}

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
 }

#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?; // Buildin command provides a nice button menu
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Dotenv crate automatically loads environment variables specified in `.env` into the environment
    env_logger::init();
    let songbird = songbird::Songbird::serenity();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::hello(), commands::stats(), register()], // We specify the commands in an 'array' (vec in rust), we then load the default values for the framework for the rest
            event_handler: |ctx, event, framework, data| {
				Box::pin(events::listener(ctx, event, framework, data))
			},
            ..Default::default()
        })
        // Login with a bot token from the environment
        .token(env::var("DISCORD_TOKEN").expect("Fill in DISCORD_TOKEN at .env")) // .expect means we just panic (crash kinda) if its missing
        // Use a bitwise OR to add the message context intent, due to intents stored as 53-bit integer bitfield
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |_ctx, _ready, _framework| {
            log::info!("Creating framework");
            Box::pin(async move { Ok(
                Data {
                    songbird: songbird.clone(),
                    bot_start_time: std::time::Instant::now(),
                    bot_user_id: _ready.user.id,
                    version: env!("CARGO_PKG_VERSION").to_string(),
                }
            )})
        })
        .client_settings(move |f| f
            .
            .event_handler(EventHandler {framework: framework_oc_clone, fully_started: AtomicBool::new(false)}));
    /////////////////////
    log::info!("Running bot");
    framework.run_autosharded().await.unwrap();
}