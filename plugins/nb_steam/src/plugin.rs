// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::collections::HashMap;

use chrono::Utc;
use itertools::Itertools;
use nb_fluent::FluentFramework;
use nb_poise::{Ctx, PoiseFramework};
use nbf::{Framework, Plugin, PluginLoader, R};
use nbl_steam_api::{SteamAPI, OwnedGame};
use rust_embed::RustEmbed;
use sea_query::{PostgresQueryBuilder, OnConflict, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Postgres, PgPool, FromRow};

use crate::cmd;
pub use crate::schema::*;

pub struct SteamPlugin {
  pub api_key: String,
}

impl Default for SteamPlugin {
  fn default() -> Self {
    Self {
      api_key: std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY is not set"),
    }
  }
}

impl Plugin for SteamPlugin {
  fn init(self, fw: &mut Framework) -> R {
    // Technically not needed as we already depend on DiscordPlugin
    fw.require_plugin::<nb_sqlx::SqlxPlugin<Postgres>>()?;
    fw.require_plugin::<nb_discord::DiscordPlugin>()?;
    fw.state.put(SteamAPI::new(&self.api_key));
    fw.require_plugin::<nb_fluent::FluentPlugin>()?;
    fw.add_fluent_resources::<SteamLocale>()?;
    fw.require_plugin::<nb_poise::PoisePlugin>()?;
    fw.add_command(cmd::steam())?;
    Ok(())
  }
}



#[derive(RustEmbed)]
#[folder = "locale"]
struct SteamLocale;

const CHUNK_SIZE: usize = 1000;

#[derive(FromRow, Debug)]
#[allow(dead_code)]
struct AccountRow {
  id: i64,
}

pub(crate) async fn refresh_single_steam_user(api: &SteamAPI, p: &PgPool, acc: i64) -> R {
  log::info!("Refreshing Steam user");
  if let Ok(res) = api.get_owned_games(&acc).await {
    let games = res.response.games;
    refresh_apps(games.iter().collect(), p).await?;
    refresh_playdata(
      games
        .iter()
        .map(move |o| (o.appid, acc, o.playtime_forever))
        .collect::<Vec<_>>(),
      p,
    )
    .await?;
  } else {
    log::warn!("Issue updating stats for {0}", acc);
  }
  Ok(())
}

pub(crate) async fn refresh_steam_data(api: &SteamAPI, p: &PgPool) -> R {
  log::info!("Refreshing Steam data");
  let (q, v) = Query::select()
    .column(Accounts::id)
    .from(Accounts::Table)
    .build_sqlx(PostgresQueryBuilder);
  let mut games: HashMap<i64, Vec<_>> = HashMap::new();
  for r in sqlx::query_as_with::<_, AccountRow, _>(&q, v)
    .fetch_all(p)
    .await?
  {
    if let Ok(res) = api.get_owned_games(&r.id).await {
      games.insert(r.id, res.response.games);
    } else {
      log::warn!("Issue updating stats for {0}", r.id);
    }
  }
  refresh_apps(
    games
      .iter()
      .flat_map(|(_, g)| g)
      .unique_by(|g| g.appid)
      .collect(),
    p,
  )
  .await?;
  refresh_playdata(
    games
      .iter()
      .flat_map(|(&i, g)| {
        g.into_iter()
          .map(move |o| (o.appid, i, o.playtime_forever))
          .collect::<Vec<_>>()
      })
      .collect(),
    p,
  )
  .await?;
  Ok(())
}

async fn refresh_apps(apps: Vec<&OwnedGame>, p: &PgPool) -> R {
  log::trace!("Updating {} apps", apps.len());
  for chunk in apps.chunks(CHUNK_SIZE) {
    let mut q = Query::insert()
      .into_table(Apps::Table)
      .columns([Apps::id, Apps::name])
      .on_conflict(
        OnConflict::column(Apps::id)
          .update_column(Apps::name)
          .to_owned(),
      )
      .to_owned();
    let data: Vec<_> = chunk
      .into_iter()
      .map(|a| [a.appid.into(), a.name.clone().into()])
      .collect();
    for row in data {
      q.values(row)?;
    }
    let (q, v) = q.build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&q, v).execute(p).await?;
    log::trace!("Updated {} apps", chunk.len());
  }
  Ok(())
}

#[derive(FromRow)]
#[allow(dead_code)]
struct PlayHistReturn {
  id: i32,
  mins: i32,
}

async fn refresh_playdata(playdata: Vec<(i32, i64, i32)>, p: &PgPool) -> R {
  // Yes a day, is never exactly the same, but I just need to round the timestamp to current day
  let day = (Utc::now().timestamp() / 86400) as i32;
  log::trace!("Updating {} playdata entries", playdata.len());
  for chunk in playdata.chunks(CHUNK_SIZE) {
    let mut q = Query::insert()
      .into_table(PlayData::Table)
      .columns([PlayData::app_id, PlayData::acc_id, PlayData::mins])
      .on_conflict(
        OnConflict::columns([PlayData::app_id, PlayData::acc_id])
          .update_column(PlayData::mins)
          .to_owned(),
      )
      .returning(Query::returning().columns([PlayData::id, PlayData::mins]))
      .to_owned();
    let data: Vec<_> = chunk
      .into_iter()
      .map(|p| [p.0.into(), p.1.into(), p.2.into()])
      .collect();
    for row in data {
      q.values(row)?;
    }
    let (q, v) = q.build_sqlx(PostgresQueryBuilder);
    let return_data: Vec<_> = sqlx::query_as_with::<_, PlayHistReturn, _>(&q, v)
      .fetch_all(p)
      .await?;
    log::trace!("Updated {} playdata entries", chunk.len());
    let mut nq = Query::insert()
      .into_table(PlayHist::Table)
      .columns([PlayHist::play_id, PlayHist::utc_day, PlayHist::mins])
      .on_conflict(
        OnConflict::columns([PlayHist::play_id, PlayHist::utc_day])
          .update_column(PlayHist::mins)
          .to_owned(),
      )
      .to_owned();
    for row in return_data {
      nq.values([row.id.into(), day.into(), row.mins.into()])?;
    }
    let (q, v) = nq.build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&q, v).execute(p).await?;
    log::trace!("Updated {} playhist entries", chunk.len());
  }
  Ok(())
}
