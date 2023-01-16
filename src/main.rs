mod commands;
mod events;
mod helpers;
mod checks;
mod config;
mod time;
//use crate::events::listener;
use std::{env, sync::{Arc}};

use dotenv::dotenv;
use poise::{serenity_prelude::{self as serenity, UserId}};
use reqwest::Client;
//use songbird::serenity;
use songbird::SerenityInit;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    songbird: Arc<songbird::Songbird>,

    bot_start_time: std::time::Instant,
    bot_user_id: UserId,
    version: String,
    http_client: Client
}

#[derive(Debug)]
pub enum MusicErrors {
    NoUserChannel,
    NotInGuild,
    BotNotInChannel,
}

impl std::error::Error for MusicErrors {}

impl std::fmt::Display for MusicErrors {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
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
    let songbird_clone = songbird.clone();
    let framework = poise::Framework::builder()
        .token(env::var("DISCORD_TOKEN").expect("Fill in DISCORD_TOKEN at .env"))
        // Use a bitwise OR to add the message context intent, due to intents stored as 53-bit integer bitfield
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |_ctx, _ready, _framework| {
            log::info!("Creating framework");
            Box::pin(async move { Ok(
                Data {
                    songbird: songbird_clone,
                    bot_start_time: std::time::Instant::now(),
                    bot_user_id: _ready.user.id,
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    http_client: reqwest::Client::new(),
                }
            )})
        })
        .client_settings(move |client| client
            .register_songbird_with(songbird)
            //.voice_manager_arc(songbird)
        )
        .options(poise::FrameworkOptions {
            commands: vec![commands::play(), commands::hello(), commands::stats(), register(), commands::stop(), commands::skip(), commands::nowplaying(), commands::pause(), commands::resume(), commands::teams(), commands::seek(), commands::pokemon_game()],
            event_handler: |ctx, event, framework, data| {
				Box::pin(events::listener(ctx, event, framework, data))
			},
            ..Default::default()
        });
    /////////////////////
    log::info!("Running bot");
    framework.run_autosharded().await.unwrap();
}
