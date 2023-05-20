use std::{
    process::Command,
    sync::{Arc, Weak},
};

use crate::{
    errors::ParsimonyErrors,
    helpers::{self, create_information_warning, sanitise_text, ICON_URL, INFO_EMBED_COLOUR},
    Context, Error, GuildState,
};
use dashmap::DashMap;
use futures::{pin_mut, StreamExt, TryStreamExt};
use lazy_static::lazy_static;
use poise::{CreateReply, ReplyHandle};
use regex::Regex;
use reqwest::Client;
use rspotify::{
    model::{
        AlbumId, FullTrack, PlayableItem, PlaylistId, SimplifiedArtist, SimplifiedTrack, TrackId,
    },
    prelude::BaseClient,
};
use serenity::{
    all::{GuildId, UserId},
    async_trait,
    builder::{CreateEmbed, CreateEmbedAuthor},
    utils::{EmbedMessageBuilding, MessageBuilder},
};
use songbird::{
    input::{AuxMetadata, Compose, YoutubeDl},
    tracks::TrackHandle,
    Call, Event, EventContext, EventHandler, TrackEvent,
};
use tokio::sync::{Mutex, MutexGuard};
use url::Url;

pub struct AuxMetadataHolder;
impl songbird::typemap::TypeMapKey for AuxMetadataHolder {
    type Value = AuxMetadata;
}

pub struct Requestor;
impl songbird::typemap::TypeMapKey for Requestor {
    type Value = UserId;
}

#[poise::command(slash_command, guild_only)]
pub async fn play(ctx: Context<'_>, #[description = "Song"] song: String) -> Result<(), Error> {
    let data = ctx.data();
    lazy_static! {
        static ref REGEX_SPOTIFY: Regex =           Regex::new(r"https://(?:open.)?spotify.com/([a-z]+)/([0-9a-zA-Z]{22})(?:\?.*)?").unwrap();
        /// Matches:
        /// - https://www.youtube.com/watch?v=VMtarj8Ua0s
        /// - https://www.youtube.com/watch?v=VMtarj8Ua0s&list=PLsvQxTDusPIHYWNj4qoqld9cqX8QeCXEY
        /// - https://www.youtube.com/playlist?list=PLsvQxTDusPIHYWNj4qoqld9cqX8QeCXEY
        static ref REGEX_YOUTUBE_NORMAL: Regex =    Regex::new(r"(?:https?://)?(?:www.)?youtube.com/(?:(?:watch\?v=([A-Za-z0-9_-]{11,})&?)|(?:playlist\?))(?:list=([A-Za-z0-9_-]+))?").unwrap();
        /// Matches:
        /// - https://youtu.be/L_ja4paGDXs
        /// Note: Ignores start time.
        static ref REGEX_YOUTUBE_SHORT: Regex =    Regex::new(r"(?:https?://)?(?:www.)?youtu.be/([A-Za-z0-9_-]{11,})").unwrap();

    }

    let mut reply_handle = ctx
        .send(
            CreateReply::new().embed(
                CreateEmbed::new()
                    .colour(INFO_EMBED_COLOUR)
                    .description(":mag_right: **Searching...**"),
            ),
        )
        .await?;

    // TODO: Defer / Searching message.text
    // Could probs do some raw url matching before resorting to a regex capture but.. Eh.

    // =-=-=-=-=-= Spotify =-=-=-=-=-=
    if let Some(capture) = REGEX_SPOTIFY.captures(&song) {
        let resource_type = &capture[1];
        let spotify_id = &capture[2];

        let spotify = &ctx.data().spotify_client;
        // playlist, album, track, user, artist, show. Perhaps more.
        match resource_type {
            "track" => {
                let track_id = unsafe { TrackId::from_id_unchecked(spotify_id) }; // Alphanumeric already checked by regex
                let track = spotify.track(track_id).await.unwrap();
                let src = track_to_query(track, data.http_client.clone());
                single_song_queue(&ctx, src, &mut reply_handle).await?;
            }
            "playlist" => {
                let (mut ignored_tracks, mut queued_tracks) = (0u32, 0u32);
                let playlist_id = unsafe { PlaylistId::from_id_unchecked(spotify_id) };

                let stream = spotify.playlist_items(
                    playlist_id.clone(),
                    None, //Some(r"items(track(name, artists(name)))"),
                    None,
                );
                pin_mut!(stream); // Not sure what the pin is doing tbh

                let call = helpers::join_author_vc(&ctx).await?;
                let mut handle = call.lock().await;

                while let Some(item) = stream.try_next().await.map_err(|f| {
                    error!("Error while fetching spotify playlist, {f:?}");
                    ParsimonyErrors::RequestError
                })? {
                    if let Some(PlayableItem::Track(track)) = item.track {
                        queued_tracks += 1;

                        let src = track_to_query(track, ctx.data().http_client.clone());
                        enqueue_lazy(&ctx, &mut handle, Arc::downgrade(&call), src).await;
                    } else {
                        ignored_tracks += 1;
                    }
                }

                if ignored_tracks > 0 {
                    ctx.send(create_information_warning(
                        format!("Note: The bot does not currently support the playment of episodes. These have been ignored [{}]", ignored_tracks),
                        true,
                    ))
                    .await?;
                }

                let playlist_info = spotify
                    .playlist(playlist_id, None, None) // Fields: Some("name, images, id")
                    .await
                    .unwrap();

                let source_url = format!(
                    "https://open.spotify.com/playlist/{}",
                    playlist_info.id.to_string()
                );
                dbg!(&playlist_info.images); // For some reason the image embed doesnt work with the `this is... [artist]` images. Not sure why.
                let image_url = playlist_info.images.last().map(|img| img.url.as_str());
                dbg!(image_url);

                played_queue_msg(
                    &ctx,
                    &mut reply_handle,
                    queued_tracks,
                    playlist_info.name.as_str(),
                    source_url.as_str(),
                    image_url,
                )
                .await?;
            }
            "album" => {
                let mut queued_tracks = 0u32;
                let album_id = unsafe { AlbumId::from_id_unchecked(spotify_id) };

                let stream = spotify.album_track(album_id.clone());
                pin_mut!(stream);

                let call = helpers::join_author_vc(&ctx).await?;
                let mut handle = call.lock().await;

                while let Some(track) = stream.try_next().await.map_err(|f| {
                    error!("Error while fetching spotify album, {f:?}");
                    ParsimonyErrors::RequestError
                })? {
                    info!("Queuing {}", track.name);
                    queued_tracks += 1;

                    let src = track_to_query(track, ctx.data().http_client.clone());
                    enqueue_lazy(&ctx, &mut handle, Arc::downgrade(&call), src).await;
                }

                let album_info = spotify.album(album_id).await.unwrap();
                let image = album_info.images.last().map(|img| img.url.as_str());
                let source_url = format!(
                    "https://open.spotify.com/album/{}",
                    album_info.id.to_string().rsplit(':').next().unwrap()
                );

                played_queue_msg(
                    &ctx,
                    &mut reply_handle,
                    queued_tracks,
                    album_info.name.as_str(),
                    source_url.as_str(),
                    image,
                )
                .await?;
            }
            _ => {}
        }
    // =-=-=-=-=-= Youtube =-=-=-=-=-=
    } else if let Some(captures) = REGEX_YOUTUBE_NORMAL.captures(&song) {
        let song_id = captures.get(1);
        let playlist_id = captures.get(2);
        match playlist_id {
            // =-=-=-=-=-= Playlist =-=-=-=-=-=
            Some(playlist_id) => {
                let playlist_id = playlist_id.as_str();
                let (playlist_name, song_ids) = lookup_youtube_playlist(ctx, playlist_id)?;

                let call = helpers::join_author_vc(&ctx).await?;
                let mut handle = call.lock().await;

                for id in song_ids.iter() {
                    let src = YoutubeDl::new_ytdl_like(
                        "yt-dlp",
                        data.http_client.clone(),
                        format!("https://www.youtube.com/watch?v={}", id),
                    );
                    info!("{}", id);
                    enqueue_lazy(&ctx, &mut handle, Arc::downgrade(&call), src).await;
                }
                info!("Played {playlist_name}");
                let source_url = format!("https://www.youtube.com/playlist?list={}", playlist_id);
                played_queue_msg(
                    &ctx,
                    &mut reply_handle,
                    song_ids.len() as u32,
                    "Youtube playlist",
                    source_url.as_str(),
                    None,
                )
                .await?;
            }
            // =-=-=-=-=-= No Playlist =-=-=-=-=-=
            None => {
                info!("Searching..");
                let song_id = song_id.unwrap().as_str();
                let src = YoutubeDl::new_ytdl_like(
                    "yt-dlp",
                    data.http_client.clone(),
                    song_id.to_string(),
                );
                info!("Searched");
                single_song_queue(&ctx, src, &mut reply_handle).await?;
            }
        }
    // =-=-=-=-=-= Youtu.be short =-=-=-=-=-=
    } else if let Some(captures) = REGEX_YOUTUBE_SHORT.captures(song.as_str()) {
        let song_id = captures.get(1).unwrap().as_str();
        dbg!(song_id);
        let src = YoutubeDl::new_ytdl_like("yt-dlp", data.http_client.clone(), song_id.to_string());
        dbg!(&src);
        single_song_queue(&ctx, src, &mut reply_handle).await?;
    } else if Url::parse(song.as_str()).is_ok() {
        let src = YoutubeDl::new_ytdl_like("yt-dlp", data.http_client.clone(), song);
        single_song_queue(&ctx, src, &mut reply_handle).await?;
    } else {
        let src = YoutubeDl::new_ytdl_like(
            "yt-dlp",
            data.http_client.clone(),
            format!("ytsearch:{}", song),
        );
        single_song_queue(&ctx, src, &mut reply_handle).await?;
    }
    // ctx.say("O_o").await?;
    Ok(())
}

fn track_to_query(track: impl PlayableTrack, http: Client) -> YoutubeDl {
    let artists = &track
        .artists()
        .iter()
        .map(|art| art.name.as_str())
        .collect::<Vec<&str>>()
        .join(", ");

    let search_query = format!("ytsearch:{} {}", &track.name(), artists);
    YoutubeDl::new_ytdl_like("yt-dlp", http, search_query)
}

async fn pull_metadata(track: TrackHandle, mut src: YoutubeDl) {
    match src.aux_metadata().await {
        Ok(aux) => {
            let mut typemap = track.typemap().write().await;
            typemap.insert::<AuxMetadataHolder>(aux);
            trace!("Pulled metadata for {src:?}")
        }
        Err(e) => {
            error!("Error fetching meta data {e} for {src:?}");
        }
    }
}

pub trait PlayableTrack {
    fn name(&self) -> &str;
    fn artists(&self) -> &Vec<SimplifiedArtist>;
}

impl PlayableTrack for FullTrack {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn artists(&self) -> &Vec<SimplifiedArtist> {
        &self.artists
    }
}

impl PlayableTrack for SimplifiedTrack {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn artists(&self) -> &Vec<SimplifiedArtist> {
        &self.artists
    }
}

async fn enqueue_lazy(
    ctx: &Context<'_>,
    handle: &mut MutexGuard<'_, Call>,
    call: Weak<Mutex<Call>>,
    src: YoutubeDl,
) {
    // Enqueue without preload otherwise we are here for years.
    let track_handle = handle.enqueue_with_preload(src.clone().into(), None);

    let data = ctx.data();
    track_handle
        .add_event(
            songbird::Event::Track(TrackEvent::End),
            MerlinEndTrack {
                call_lock: call,
                http_client: data.http_client.clone(),
                guild: ctx.guild_id().unwrap(),
                guild_data: data.guild_states.clone(),
            },
        )
        .unwrap();

    let mut typemap = track_handle.typemap().write().await;
    typemap.insert::<Requestor>(ctx.author().id);
    drop(typemap);
    tokio::spawn(pull_metadata(track_handle, src)); // Pull metadata on another thread so we have the songs incase they are super short, also paralellism, lets hope it doesnt rate limit me.
}

async fn single_song_queue(
    ctx: &Context<'_>,
    mut src: YoutubeDl,
    _message_handle: &mut ReplyHandle<'_>,
) -> Result<(), Error> {
    let call = helpers::join_author_vc(&ctx).await?;
    let mut handle = call.lock().await;

    let meta = src.aux_metadata().await;
    let track_handle = handle.enqueue(src.into()).await;

    let data = ctx.data();
    let weak = Arc::downgrade(&call);
    track_handle.add_event(
        songbird::Event::Track(TrackEvent::End),
        MerlinEndTrack {
            call_lock: weak,
            http_client: data.http_client.clone(),
            guild: ctx.guild_id().unwrap(),
            guild_data: data.guild_states.clone(),
        },
    )?;

    let mut typemap = track_handle.typemap().write().await;
    typemap.insert::<Requestor>(ctx.author().id);
    if let Ok(meta) = meta {
        typemap.insert::<AuxMetadataHolder>(meta);
    } else {
        todo!();
    }
    Ok(())
}

pub struct MerlinEndTrack {
    pub call_lock: Weak<Mutex<Call>>,
    pub guild: GuildId,
    pub http_client: Client,
    pub guild_data: Arc<DashMap<GuildId, GuildState>>,
}

#[async_trait]
impl EventHandler for MerlinEndTrack {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let Some(guild_data) = self.guild_data.get(&self.guild) {
            if !guild_data.loop_queue {
                return None;
            }

            if let EventContext::Track(&[(_, handle)]) = ctx {
                if let Some(meta) = handle.typemap().read().await.get::<AuxMetadataHolder>() {
                    if let Some(url) = &meta.source_url {
                        let src = YoutubeDl::new_ytdl_like(
                            "yt-dlp",
                            self.http_client.clone(),
                            url.to_string(),
                        );
                        info!("Replaying song");
                        let call_arc = self.call_lock.upgrade().unwrap();
                        let mut call = call_arc.lock().await;

                        let new_handle = call.enqueue_with_preload(src.clone().into(), None);

                        let weak = Arc::downgrade(&call_arc);
                        new_handle
                            .add_event(
                                songbird::Event::Track(TrackEvent::End),
                                MerlinEndTrack {
                                    call_lock: weak,
                                    http_client: self.http_client.clone(),
                                    guild: self.guild.clone(),
                                    guild_data: self.guild_data.clone(),
                                },
                            )
                            .unwrap();

                        tokio::spawn(pull_metadata(new_handle, src));
                    }
                }
            }
        }

        None
    }
}

fn lookup_youtube_playlist<'a>(
    _ctx: Context<'_>,
    playlist_id: &str,
) -> Result<(String, Vec<String>), Error> {
    // yt-dlp --print playlist:title --compat-options no-youtube-unavailable-videos --flat-playlist --get-id --no-warnings [ID]
    let stdout = Command::new("yt-dlp")
        .args([
            "--print",
            "playlist:title",
            "--compat-options",
            "no-youtube-unavailable-videos",
            "--flat-playlist",
            "--get-id",
            "--no-warnings",
            playlist_id,
        ])
        .output()?
        .stdout;
    let text = String::from_utf8(stdout).unwrap();

    if text.len() == 0 {
        // If 0 len, error with request.
        return Err(Box::new(ParsimonyErrors::RequestError));
    }

    let output: Vec<&str> = text.trim().split("\n").collect();
    let (playlist_name, song_ids) = output.split_last().unwrap();
    Ok((
        playlist_name.to_string(),
        song_ids.iter().map(|song_id| song_id.to_string()).collect(),
    ))
}

async fn played_queue_msg(
    ctx: &Context<'_>,
    reply_handle: &mut ReplyHandle<'_>,
    song_count: u32,
    title: &str,
    source_url: &str,
    image_url: Option<&str>,
) -> Result<(), Error> {
    dbg!(image_url);
    reply_handle
        .edit(
            *ctx,
            CreateReply::new().embed(|| -> CreateEmbed {
                let mut e = CreateEmbed::new();
                e = e
                    .colour(INFO_EMBED_COLOUR)
                    .author(CreateEmbedAuthor::new("Added to queue").icon_url(ICON_URL))
                    .field("Added by", ctx.author().to_string(), true)
                    .field("Amount of songs:", format!("`{song_count}`"), true)
                    .url(source_url)
                    .description(
                        MessageBuilder::new()
                            .push_named_link_safe(sanitise_text(title), source_url)
                            .build(),
                    );

                if let Some(url) = image_url {
                    e = e.thumbnail(url);
                };

                e
            }()),
        )
        .await?;
    Ok(())
}
