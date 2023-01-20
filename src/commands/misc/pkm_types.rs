
use std::str::FromStr;

use poise::serenity_prelude::{Channel, UserId, User, Colour};

use rand::seq::SliceRandom;


use crate::{Context, Error, helpers};
use crate::helpers::*;

struct PkmType(String, Colour);

const EMBED_COLOUR: Colour = Colour::from_rgb(0,170,255);

async fn default_vc(ctx: &Context<'_>, specified: Option<Channel>) -> Option<Channel> {
    if let Some(channel) = specified {
        return Some(channel);
    }

    return match helpers::get_user_vc(ctx) {
        // We pray to the unwrap gods
        Some(c) => Some(c.to_channel(ctx.serenity_context()).await.unwrap()),
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
                helpers::send_information_warning(ctx, format!("Could not parse {} to userID", individual), true).await.unwrap();
                continue;
            },
        };

        let user = ctx.serenity_context().http.get_user(user_id.0).await;
        match user {
            Ok(u) => {
                list.push(u);
            },
            Err(_) => {
                helpers::send_information_warning(ctx, format!("Could not find user from UserID {}", individual), true).await.unwrap();
                continue;
            },
        }
    }
    return list;
}

#[poise::command(
    slash_command,
    guild_only,
    global_cooldown = 20,
)]
pub async fn pokemon_game(
    ctx: Context<'_>,
    #[description = "If types should be dmed and kept secret"]
    secret: bool,
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
            helpers::generic_error(&ctx, "You must either be in a voice channel or specify an override to use this command").await?;
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
    let people = guild_vc.members(ctx.serenity_context()).await?;
    let mut user_list: Vec<User> = Vec::new();
    for member in people {
        if member.user.bot == false {
            if exclude.contains(&member.user) == false {
                user_list.push(member.user);
            }
        }
    }
    
    let mut types = vec![
        PkmType("Normal".to_string(), Colour::from_rgb(168, 168, 120)),
        PkmType("Fighting".to_string(), Colour::from_rgb(192, 48, 40)),
        PkmType("Flying".to_string(), Colour::from_rgb(168, 144, 240)),
        PkmType("Poison".to_string(), Colour::from_rgb(160, 64, 160)),
        PkmType("Ground".to_string(), Colour::from_rgb(184, 160, 56)),
        PkmType("Rock".to_string(), Colour::from_rgb(184, 160, 56)),
        PkmType("Bug".to_string(), Colour::from_rgb(168, 184, 32)),
        PkmType("Ghost".to_string(), Colour::from_rgb(112, 88, 152)),
        PkmType("Steel".to_string(), Colour::from_rgb(184, 184, 208)),
        PkmType("Fire".to_string(), Colour::from_rgb(240, 128, 48)),
        PkmType("Water".to_string(), Colour::from_rgb(104, 144, 240)),
        PkmType("Grass".to_string(), Colour::from_rgb(120, 200, 80)),
        PkmType("Electric".to_string(), Colour::from_rgb(248, 208, 48)),
        PkmType("Psychic".to_string(), Colour::from_rgb(248, 88, 136)),
        PkmType("Ice".to_string(), Colour::from_rgb(152, 216, 216)),
        PkmType("Dragon".to_string(), Colour::from_rgb(112, 56, 248)),
        PkmType("Dark".to_string(), Colour::from_rgb(112, 88, 72)),
        PkmType("Fairy".to_string(), Colour::from_rgb(238, 153, 172)),
        ];
    {
        {
        let mut rng = rand::thread_rng();
        user_list.shuffle(&mut rng);
        types.shuffle(&mut rng);
        }

        for (i,person) in user_list.into_iter().enumerate() {
            let pkm_type = &types[i&types.len()];
            match secret {
                true => {
                    let resp = person.direct_message(&ctx, |m| {
                        m.add_embed(|e| {
                            e.title(format!("Pokemon Type Game: {}", &pkm_type.0))
                            .colour(pkm_type.1)
                            .description(format!("Hey {}, your type is **{}**",person, pkm_type.0))
                        })
                    }).await;
                    resp?;
                    send_clear_embed(&ctx, "**✅ | Sent message**").await?;
                },
                false => {
                    let mut lines = String::new();
                    lines.push_str(&format!("{} - **{}**\n",person, pkm_type.0));
                    send_simple_embed(&ctx, EMBED_COLOUR, lines).await?;

                }
            }
        }
    }
    //user_list.choose_multiple(&mut rng, amount);
    Ok(())
}