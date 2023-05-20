#[macro_use]
extern crate log;

pub mod commands;
pub mod config;
pub mod errors;
pub mod events;
pub mod helpers;

use config::Config;
use dashmap::DashMap;
use poise::{Framework, FrameworkOptions};
use rspotify::{ClientCredsSpotify, Credentials};
use serenity::{
    all::{GuildId, UserId},
    prelude::GatewayIntents,
};
use songbird::{SerenityInit, Songbird};
use std::sync::Arc;
use tokio::time::Instant;

use crate::config::load_config;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let config = load_config("config.ron".into());
    pretty_env_logger::init();

    let songbird = songbird::Songbird::serenity();
    let songbird_clone = songbird.clone();
    let token = config.bot_token.clone();
    let intents = GatewayIntents::non_privileged();

    let framework = Framework::new(
        FrameworkOptions {
            commands: vec![
                register(),
                commands::join(),
                commands::play(),
                commands::loops(),
            ],
            listener: |event, framework, data| Box::pin(events::listener(event, framework, data)),
            ..Default::default()
        },
        move |_ctx, ready, _framework| {
            info!("Creating framework");

            let mut spotify_client = ClientCredsSpotify::new(Credentials::new(
                &config.spotify_settings.client_id,
                &config.spotify_settings.client_secret,
            ));
            spotify_client.config.token_refreshing = true;

            Box::pin(async move {
                spotify_client.request_token().await.unwrap();
                Ok(Data {
                    bot_user_id: ready.user.id,
                    songbird: songbird_clone,
                    http_client: reqwest::Client::new(),
                    guild_states: Arc::new(DashMap::new()),
                    spotify_client,
                    config,
                })
            })
        },
    );

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .unwrap();

    info!("Running bot");
    client.start_autosharded().await.unwrap();
}

#[derive(Debug)]
pub struct Data {
    config: Config,
    bot_user_id: UserId,
    songbird: Arc<Songbird>,
    http_client: reqwest::Client,
    guild_states: Arc<DashMap<GuildId, GuildState>>,
    spotify_client: ClientCredsSpotify,
}

#[derive(Debug, Default)]
pub struct Timeouts {
    lonely_leavetime: Option<Instant>,
    idle_leavetime: Option<Instant>,
}

#[derive(Debug, Default)]
pub struct GuildState {
    pub timeouts: Timeouts,
    pub loop_queue: bool,
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
