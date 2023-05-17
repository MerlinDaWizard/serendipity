use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub bot_token: String,
    /// An env logger string which we load in. Weird work around I guess but it works
    pub logging: String,
    /// Should the bot pause when everyone leaves the channel?
    pub auto_pause: bool,
    pub voice_settings: VoiceSettings,
    #[cfg(feature = "spotify")]
    pub spotify_settings: SpotifySettings,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct VoiceSettings {
    /// When the bot finishes playing its queue, what should it do
    pub on_idle: DisconnectOptions,
    /// When a bot is left alone in its voice channel, what should it do
    pub on_lonely: DisconnectOptions,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum DisconnectOptions {
    /// Set a timer with the amount of seconds as configured
    Timeout(usize),
    /// Instantly disconnect
    Instant,
    /// Bot will not disconnect due to the respective action
    Off,
}

#[derive(Debug, Deserialize, Serialize)]
/// <https://developer.spotify.com/dashboard>
pub struct SpotifySettings {
    pub access_token_url: String,
    pub client_id: String,
    pub client_secret: String,
}

/// Set the logging due to env_logger being... env... logger
pub fn load_config(path: PathBuf) -> Config {
    let text = fs::read_to_string(path).unwrap();
    let config: Config = ron::from_str(&text).unwrap();
    std::env::set_var("RUST_LOG", &config.logging);
    config
}
