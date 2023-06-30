// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nb_lifecycle::LifecycleFramework;
use nb_poise::{Ctx, PoiseFramework};
use nb_steam::{Accounts, PlayData};
use nbf::{Framework, PluginLoader, Res, R};
use poise::{
  futures_util::StreamExt,
  serenity_prelude::{ChannelId, Colour, Role, RoleId, User},
  Event,
};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, PgPool};

#[tokio::main]
async fn main() -> R {
  if !std::env::var("RUST_LOG").is_ok_and(|f| !f.is_empty()) {
    std::env::set_var("RUST_LOG", "warn");
  }
  pretty_env_logger::init();
  let mut fw = Framework::new();
  fw.init_plugin(nb_steam::SteamPlugin::default())?;

  fw.add_event_handler(|ctx, e, _fctx, s| {
    Box::pin(async move {
      let db = s.read().await.borrow::<PgPool>()?.clone();
      match e {
        // User joined
        Event::GuildMemberAddition { new_member } => {
          ChannelId(1038997543515865148)
            .send_message(ctx, |m| {
              let user = &new_member.user;
              m.embed(|e| {
                e.author(|a| {
                  a.icon_url(get_avatar(&user));
                  a.name(if user.discriminator != 0 {
                    format!("{}#{:0>4}", user.name, user.discriminator)
                  } else {
                    user.name.clone()
                  });
                  a.url(format!("https://discord.com/users/{}", user.id))
                });
                e.colour(Colour::from_rgb(139, 195, 74));
                e.description(format!("Welcome <@{}> to the server!", user.id))
              })
              .components(|c| {
                c.create_action_row(|r| {
                  r.create_button(|b| {
                    b.url(format!("https://discord.com/users/{}", user.id))
                      .label("Profile")
                  })
                })
              })
            })
            .await?;
          new_member
            .guild_id
            .member(ctx, new_member.user.id)
            .await?
            .add_roles(
              ctx,
              get_game_roles(query_games(&db, new_member.user.id.0).await?).as_slice(),
            )
            .await?;
        }
        Event::GuildMemberRemoval {
          guild_id: _,
          user,
          member_data_if_available: _,
        } => {
          ChannelId(1038997543515865148)
            .send_message(ctx, |m| {
              m.embed(|e| {
                e.author(|a| {
                  a.icon_url(get_avatar(&user));
                  a.name(if user.discriminator != 0 {
                    format!("{}#{:0>4}", user.name, user.discriminator)
                  } else {
                    user.name.clone()
                  });
                  a.url(format!("https://discord.com/users/{}", user.id))
                });
                e.colour(Colour::from_rgb(244, 67, 54));
                e.description(format!("<@{}> has left the server!", user.id))
              })
              .components(|c| {
                c.create_action_row(|r| {
                  r.create_button(|b| {
                    b.url(format!("https://discord.com/users/{}", user.id))
                      .label("Profile")
                  })
                })
              })
            })
            .await?;
        }
        _ => {}
      }
      Ok(())
    })
  })?;

  fw.add_command(update_roles())?;
  // WIP
  fw.init_plugin(nb_anilist::AnilistPlugin)?;
  fw.init_plugin(nb_vrchat::VRCPlugin)?;

  fw.run().await?;
  Ok(())
}

#[poise::command(slash_command, owners_only)]
async fn update_roles(ctx: Ctx<'_>) -> R {
  let db = ctx.data().read().await.borrow::<PgPool>()?.clone();
  if let Some(g) = ctx.guild_id() {
    let mut members = g.members_iter(&ctx).boxed();
    while let Some(member_result) = members.next().await {
      match member_result {
        Ok(mut member) => {
          member
            .add_roles(
              ctx,
              get_game_roles(query_games(&db, member.user.id.0).await?).as_slice(),
            )
            .await?;
        }
        Err(error) => {
          log::warn!("error moment");
        }
      }
    }
  }
  Ok(())
}

#[derive(FromRow, Clone)]
#[allow(dead_code)]
pub struct GameRow {
  pub id: i32,
}

async fn query_games(db: &PgPool, u: u64) -> Res<Vec<GameRow>> {
  let mut qb = Query::select();
  qb.from(Accounts::Table);
  qb.from(PlayData::Table);
  qb.and_where(
    Expr::col((Accounts::Table, Accounts::id)).equals((PlayData::Table, PlayData::acc_id)),
  );
  qb.and_where(Expr::col((Accounts::Table, Accounts::discord_id)).eq(u as i64));
  qb.distinct();
  let (q, v) = qb
    .column((PlayData::Table, PlayData::app_id))
    .build_sqlx(PostgresQueryBuilder);
  Ok(
    sqlx::query_as_with::<_, GameRow, _>(&q, v)
      .fetch_all(db)
      .await?,
  )
}

//TODO: Add top games by amount of people who own game

fn get_game_roles(games: Vec<GameRow>) -> Vec<RoleId> {
  games
    .iter()
    .filter_map(|f| match f.id {
      730 => Some(1124283633415503983),
      427520 => Some(1124283636942905344),
      227300 => Some(1124283645109219468),
      281990 => Some(1124283635579768902),
      438100 => Some(1124283652814159992),
      620980 => Some(1124283640440963193),
      105600 => Some(1124283650851213362),
      244850 => Some(1124283646342352967),
      _ => None,
    })
    .map(|i| RoleId(i))
    .collect()
}

fn get_avatar(u: &User) -> String {
  if let Some(avatar) = u.avatar_url() {
    return avatar;
  }
  u.default_avatar_url()
}
