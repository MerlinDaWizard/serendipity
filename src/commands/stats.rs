
use poise::{serenity_prelude::{self as serenity, CreateEmbedFooter, EmbedFooter, Colour, ShardId, CreateEmbed}, CreateReply};
use crate::{Context, Error, built_info};

const DELIMETER: char = '・';
struct TimeData {
    years: u16,
    days: u16,
    hours: u8,
    mins: u8,
    secs: u8,
}

const YEAR: u64 = 365*24*60*60; // Yes we ignore leap years and stuff, sue me
const DAY: u64 = 24*60*60; // Stored as u64 just to keep arithmetic kinda simple
const HOUR: u64 = 60*60;
const MINUTE: u64 = 60;

// Could use human time instead but ¯\_(ツ)_/¯
impl TimeData {
    fn new(time_delta: u64) -> TimeData {
        TimeData {
            years: (time_delta / YEAR) as u16,
            days: (time_delta % YEAR / DAY) as u16,
            hours: (time_delta % DAY / HOUR) as u8,
            mins: (time_delta % HOUR / MINUTE) as u8,
            secs: (time_delta % MINUTE) as u8,
        }
    }

    fn format(&self) -> String {
        let mut formatted = String::new();
        let mut after = false;
        if self.years != 0 {
            formatted.push_str(&format!("{} Yrs", self.years));
            after = true;
        }
        if after || self.days != 0 {
            if after {formatted.push(DELIMETER)} else {after = true}
            formatted.push_str(&format!("{} Days", self.days));
        }
        if after || self.hours != 0 {
            if after {formatted.push(DELIMETER)} else {after = true}
            formatted.push_str(&format!("{} Hrs", self.hours));        }
        if after || self.mins != 0 {
            if after {formatted.push(DELIMETER)} else {after = true}
            formatted.push_str(&format!("{} Mins", self.mins));
        }
        if after || self.secs != 0 {
            if after {formatted.push(DELIMETER)}
            formatted.push_str(&format!("{} Secs", self.secs))
        }
        return formatted;
    }
}

async fn get_system_uptime() -> String {
    match uptime_lib::get() {
        Ok(uptime) => {
           return TimeData::new(uptime.as_secs()).format();
        }
        Err(err) => {
            eprintln!("Error getting uptime: {}", err);
            return "Err".to_string();
        }
    }
}

/// Returns some system stats and uptime data
#[poise::command(slash_command)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    //ctx.cache.
    let bot = ctx.data().bot_user_id.to_user(ctx).await.unwrap();
    
    let colour = Colour::new(0x2f3136); // Hide the side colour by making it the same as the background
    let shard_num = ctx.serenity_context().cache.shard_count();
    let guild_num = ctx.serenity_context().cache.guild_count();
    let shard_id = ctx.serenity_context().shard_id;
    // This should burn in holy fire
    let shard_latency = ctx.framework().shard_manager.lock().await.runners.lock().await[&ShardId(shard_id)].latency;
    let latency_msg = match shard_latency {
        Some(duration) => format!("{}ms",duration.as_millis()),
        None => "NYA".to_string(),
    };

    let bot_uptime = std::time::Instant::now() - ctx.data().bot_start_time;
    let bot_uptime_formatted = TimeData::new(bot_uptime.as_secs()).format();
    let ver_full = built_info::RUSTC_VERSION.split(' ').collect::<Vec<&str>>();
    let mut ver_hash = ver_full[2].to_string();
    ver_hash.remove(0); // Get rid of bracket
    let ver_num = ver_full.get(1).unwrap_or(&"Unknown");
    let host = built_info::HOST;
    // Iterative compiles kinda break this when doings lots of commits / compiles but it should work after cargo clean
    let hash = built_info::GIT_COMMIT_HASH.unwrap_or_else(|| "Unknown");
    let sys_uptime = get_system_uptime().await;
    
    //ctx.say(get_system_uptime().await).await?;
    ctx.send(|reply| reply
        .embed(|e| e
            .title(format!("{} Information", bot.name))
            .colour(colour)
            .description(format!("```yml\nName: {name}#{descrim} [{id}]\nAPI: {latency_msg}\nRuntime: {bot_uptime_formatted}```", name=bot.name, descrim = bot.discriminator, id = bot.id))
            .fields(vec![
                ("Process stats",format!("```yml\nUptime: {bot_uptime_formatted}\nRustc: {ver_num} {ver_hash}\nRAM: TODO```"), true),
                ("Bot stats", format!("```yml\nGuilds: {guild_num}\nShards: {shard_num}\nVer: {}```", &ctx.data().version), true),
                ("System stats", format!("```yml\nHost: {host}\nUptime: {sys_uptime}```"), false)
                ])
            //.footer(|f| (format!("Build {}", hash)))
            .footer(|f| f.text(format!("Build {}", hash)))
        )
    ).await?;
    Ok(())
}
