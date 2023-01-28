use std::num::NonZeroUsize;
use std::str::FromStr;

use poise::serenity_prelude::{Channel, UserId, User, Colour};

use rand::seq::SliceRandom;


use crate::{Context, Error, helpers};
use crate::helpers::*;

const EMBED_COLOUR: Colour = Colour::from_rgb(0,170,255);

async fn default_vc(ctx: &Context<'_>, specified: Option<Channel>) -> Option<Channel> {
    if let Some(channel) = specified {
        return Some(channel);
    }

    return match helpers::get_user_vc(ctx) {
        // We pray to the unwrap gods
        Some(c) => Some(c.to_channel(ctx.discord()).await.unwrap()),
        None => None,
    };
}

async fn parse_users(ctx: &Context<'_>, input: String) -> Vec<User> {
    let trimmed = input.trim();
    let mut list: Vec<User> = Vec::new();
    for individual in trimmed.split(' ') {
        let user_id = match UserId::from_str(individual) {
            Ok(uid) => uid,
            Err(_) => {
                ctx.send(create_information_warning(format!("Could not parse {} to userID", individual), true).await).await;
                continue;
            },
        };

        let user = ctx.discord().http.get_user(user_id).await;
        match user {
            Ok(u) => {
                list.push(u);
            },
            Err(_) => {
                ctx.send(create_information_warning(format!("Could not find user from UserID {}", individual), true).await).await;
                continue;
            },
        }
    }
    return list;
}

#[poise::command(
    slash_command,
    guild_only,
)]
pub async fn teams(
    ctx: Context<'_>,
    #[description = "Number of teams"]
    team_count: NonZeroUsize,
    #[description = "Voice channel to act on, defaults to user's"]
    #[channel_types("Voice")]
    voice_channel: Option<Channel>,
    #[description = "Anyone to exlude from the list"]
    exclude: Option<String>,
) -> Result<(), Error> {
    let _guild_id = ctx.guild_id().unwrap();

    let voice_channel = match default_vc(&ctx, voice_channel).await {
        Some(c) => c,
        None => {
            ctx.send(create_generic_error("You must either be in a voice channel or specify an override to use this command").await).await?;
            return Ok(());
        }
    };

    let exclude: Vec<User> = match exclude {
        Some(e) => {
            let a = parse_users(&ctx, e).await;
            a
        },
        None => {
            Vec::new()
        }
    };

    let guild_vc = voice_channel.guild().unwrap();
    let people = guild_vc.members(ctx.discord())?;
    let mut user_list: Vec<User> = Vec::new();
    for member in people {
        if member.user.bot == false {
            if exclude.contains(&member.user) == false {
                user_list.push(member.user);
            }
        }
    }

    let mut team_sizes = vec![user_list.len()/team_count.get(); team_count.get()];
    for n in 0..user_list.len()%team_count.get() {
        team_sizes[n] += 1;
    }
    
    let mut teams: Vec<Vec<&User>> = vec![Vec::new(); team_count.get()];
    {
        let mut rng = rand::thread_rng();  
        let mut prev = 0usize;
        user_list.shuffle(&mut rng);
        for (i,size) in team_sizes.into_iter().enumerate() {

            for idx in prev..prev+size {
                teams[i].push(&user_list[idx]);
            }
            prev += size;
        }
    }
    //user_list.choose_multiple(&mut rng, amount);
    let mut lines: Vec<String> = Vec::with_capacity(team_count.get());
    for (i,t) in teams.iter().enumerate() {
        let mut line = String::new();
        line.push_str(&format!("Team {}: ", i+1));
        if t.len() > 0 {
            line.push_str(&itertools::join(t, ", "));
        } else {
            line.push_str("***empty***")
        }
        lines.push(line);
    }
    ctx.send(create_simple_embed(EMBED_COLOUR, itertools::join(lines, "\n")).await).await?;
    Ok(())
}