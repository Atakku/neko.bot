// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nb_poise::PoiseFramework;
use nbf::{Framework, Plugin, PluginLoader, R};
use poise::{
  futures_util::StreamExt,
  serenity_prelude::{Context, GatewayIntents, GuildId, Member, User, UserId},
  Event,
};
use sea_query::{Cond, Expr, OnConflict, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, PgPool, Pool, Postgres};

use crate::schema::*;

pub mod schema;

pub struct DiscordPlugin;

impl Default for DiscordPlugin {
  fn default() -> Self {
    Self {}
  }
}

impl Plugin for DiscordPlugin {
  fn init(self, fw: &mut Framework) -> R {
    fw.require_plugin::<nb_poise::PoisePlugin>()?;
    fw.require_plugin::<nb_sqlx::SqlxPlugin>()?;
    fw.add_intents(GatewayIntents::GUILDS)?;
    fw.add_intents(GatewayIntents::GUILD_MEMBERS)?;
    fw.add_event_handler(|ctx, e, _fctx, s| {
      Box::pin(async move {
        let db = s.read().await.borrow::<PgPool>()?.clone();
        match e {
          // Bot started or reconnected. Full sync
          Event::Ready { data_about_bot: _ } => {
            log::trace!("Triggered full update due to Event::Ready()");
            full_update(ctx, &db).await?
          }
          // Bot added to guild
          Event::GuildCreate { guild, is_new: _ } => {
            log::trace!(
              "Triggered guild update due to Event::GuildCreate({})",
              guild.id
            );
            update_guild(guild.id, ctx, &db).await?
          }
          // Guild information updated
          Event::GuildUpdate {
            old_data_if_available: _,
            new_but_incomplete,
          } => {
            log::trace!(
              "Triggered guild info update due to Event::GuildUpdate({})",
              new_but_incomplete.id
            );
            update_guild_information(new_but_incomplete.id, ctx, &db).await?
          }
          // Bot removed from guild
          Event::GuildDelete {
            incomplete,
            full: _,
          } => {
            log::trace!(
              "Triggered guild update due to Event::GuildDelete({})",
              incomplete.id
            );
            update_guild(incomplete.id, ctx, &db).await?
          }
          // Member joined
          Event::GuildMemberAddition { new_member } => {
            if !new_member.user.bot {
              log::trace!(
                "Triggered user & member update due to Event::GuildMemberAddition({})",
                new_member.user.id
              );
              update_users(vec![new_member.user.clone()], &db).await?;
              update_members(vec![new_member.clone()], &db).await?;
            }
          }
          // Member update
          Event::GuildMemberUpdate {
            old_if_available: _,
            new,
          } => {
            if !new.user.bot {
              log::trace!(
                "Triggered member update due to Event::GuildMemberUpdate({})",
                new.user.id
              );
              update_members(vec![new.clone()], &db).await?
            }
          }
          // Member left
          Event::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available: _,
          } => {
            log::trace!(
              "Triggered member removal due to Event::GuildMemberRemoval({})",
              user.id
            );
            remove_member(*guild_id, user.id, &db).await?
          }
          _ => {}
        }
        Ok(())
      })
    })?;
    Ok(())
  }
}

const CHUNK_SIZE: usize = 10000;

#[derive(FromRow)]
#[allow(dead_code)]
struct WhitelistRow {
  id: i64,
}

// Runs on startup and reconnect
async fn full_update(ctx: &Context, p: &Pool<Postgres>) -> R {
  // TODO: leave from unwhitelisted
  log::info!("Starting full update");
  let (q, v) = Query::select()
    .column(Whitelist::id)
    .from(Whitelist::Table)
    .build_sqlx(PostgresQueryBuilder);
  let whitelist: Vec<_> = sqlx::query_as_with::<_, WhitelistRow, _>(&q, v)
    .fetch_all(p)
    .await?;
  for entry in whitelist {
    update_guild(GuildId(entry.id as u64), ctx, p).await?;
  }
  log::info!("Finished full update");
  Ok(())
}

// Runs on startup, reconnects and on guild joins
async fn update_guild(id: GuildId, ctx: &Context, p: &Pool<Postgres>) -> R {
  log::trace!("Updating guild '{}'", id);
  if let Err(err) = update_guild_information(id, ctx, p).await {
    log::warn!(
      "Can't fetch or insert guild information for id '{}': {}",
      id,
      err
    );
    remove_guild_information(id, p).await?;
    // No need to remove members as the previous update cascades
  } else {
    let res: Vec<_> = id.members_iter(ctx).collect().await;
    log::trace!("Fetched guild members");
    let members: Vec<_> = res
      .into_iter()
      .filter_map(Result::ok)
      .filter(|m| !m.user.bot)
      .collect();
    let users: Vec<_> = members.clone().into_iter().map(|m| m.user).collect();
    update_users(users, p).await?;
    remove_members(id, p).await?;
    update_members(members, p).await?;
  }
  log::trace!("Done updating guild");
  Ok(())
}

async fn update_guild_information(id: GuildId, ctx: &Context, p: &Pool<Postgres>) -> R {
  log::trace!("Fetching information for guild '{}'", id);
  let info = id.get_preview(ctx).await?;
  log::trace!("Fetched guild information");
  let (q, v) = Query::insert()
    .into_table(Guilds::Table)
    .columns([Guilds::id, Guilds::name, Guilds::icon])
    .on_conflict(
      OnConflict::column(Guilds::id)
        .update_columns([Guilds::name, Guilds::icon])
        .to_owned(),
    )
    .values([info.id.0.into(), info.name.into(), info.icon.into()])?
    .build_sqlx(PostgresQueryBuilder);
  sqlx::query_with(&q, v).execute(p).await?;
  log::trace!("Done updating guild information");
  Ok(())
}

async fn remove_guild_information(id: GuildId, p: &Pool<Postgres>) -> R {
  log::trace!("Removing guild information for guild '{}'", id);
  let (q, v) = Query::delete()
    .from_table(Guilds::Table)
    .cond_where(Expr::col(Guilds::id).eq(id.0))
    .build_sqlx(PostgresQueryBuilder);
  sqlx::query_with(&q, v).execute(p).await?;
  log::trace!("Done removing guild information");
  Ok(())
}

// Runs on guild update and on user join
async fn update_users(users: Vec<User>, p: &Pool<Postgres>) -> R {
  log::trace!("Updating {} users", users.len());
  for chunk in users.chunks(CHUNK_SIZE) {
    let mut q = Query::insert()
      .into_table(Users::Table)
      .columns([Users::id, Users::username, Users::nickname, Users::avatar])
      .on_conflict(
        OnConflict::column(Users::id)
          .update_columns([Users::username, Users::nickname, Users::avatar])
          .to_owned(),
      )
      .clone();
    let data: Vec<_> = chunk
      .into_iter()
      .map(|u| {
        [
          u.id.0.into(),
          u.name.clone().into(),
          None::<String>.into(),
          u.avatar.clone().into(),
        ]
      })
      .collect();
    for row in data {
      q.values(row)?;
    }
    let (q, v) = q.build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&q, v).execute(p).await?;
    log::trace!("Updated {} users", chunk.len());
  }
  Ok(())
}

async fn update_members(members: Vec<Member>, p: &Pool<Postgres>) -> R {
  log::trace!("Updating {} members", members.len());
  for chunk in members.chunks(CHUNK_SIZE) {
    let mut q = Query::insert()
      .into_table(Members::Table)
      .columns([
        Members::guild_id,
        Members::user_id,
        Members::nickname,
        Members::avatar,
      ])
      .on_conflict(
        OnConflict::columns([Members::guild_id, Members::user_id])
          .update_columns([Members::nickname, Members::avatar])
          .to_owned(),
      )
      .clone();
    let data: Vec<_> = chunk
      .into_iter()
      .map(|m| {
        [
          m.guild_id.0.into(),
          m.user.id.0.into(),
          m.nick.clone().into(),
          m.avatar.clone().into(),
        ]
      })
      .collect();
    for row in data {
      q.values(row)?;
    }
    let (q, v) = q.build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&q, v).execute(p).await?;
    log::trace!("Updated {} members", chunk.len());
  }
  Ok(())
}

async fn remove_members(id: GuildId, p: &Pool<Postgres>) -> R {
  log::trace!("Removing all members from guild '{}'", id);
  let (q, v) = Query::delete()
    .from_table(Members::Table)
    .cond_where(Expr::col(Members::guild_id).eq(id.0))
    .build_sqlx(PostgresQueryBuilder);
  sqlx::query_with(&q, v).execute(p).await?;
  log::trace!("Done removing all members from guild");
  Ok(())
}

async fn remove_member(gid: GuildId, uid: UserId, p: &Pool<Postgres>) -> R {
  log::trace!("Removing member '{}' from guild '{}'", uid, gid);
  let (q, v) = Query::delete()
    .from_table(Members::Table)
    .cond_where(
      Cond::all()
        .add(Expr::col(Members::guild_id).eq(gid.0))
        .add(Expr::col(Members::user_id).eq(uid.0)),
    )
    .build_sqlx(PostgresQueryBuilder);
  sqlx::query_with(&q, v).execute(p).await?;
  log::trace!("Done removing member from guild");
  Ok(())
}
