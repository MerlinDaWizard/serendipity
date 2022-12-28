


use poise::{serenity_prelude::{Colour, ShardId}};
use crate::{Context, Error, built_info};

use crate::time::DurationFormatter;

async fn get_system_uptime() -> String {
    match uptime_lib::get() {
        Ok(uptime) => {
           DurationFormatter::new(&uptime).format_long()
        }
        Err(err) => {
            eprintln!("Error getting uptime: {}", err);
            "Err".to_string()
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
    let bot_uptime_formatted = DurationFormatter::new(&bot_uptime).format_long();
    let ver_full = built_info::RUSTC_VERSION.split(' ').collect::<Vec<&str>>();
    let mut ver_hash = ver_full[2].to_string();
    ver_hash.remove(0); // Get rid of bracket
    let ver_num = ver_full.get(1).unwrap_or(&"Unknown");
    let host = built_info::HOST;
    // Iterative compiles kinda break this when doings lots of commits / compiles but it should work after cargo clean
    let hash = built_info::GIT_COMMIT_HASH.unwrap_or("Unknown");
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
