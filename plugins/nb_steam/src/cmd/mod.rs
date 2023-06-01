// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

//TODO: REWRITE
use nb_poise::Ctx;
use nbf::R;
use nbl_steam_api::SteamAPI;
use sqlx::PgPool;

use crate::refresh_steam_data;

mod app;
mod link;
pub(crate) mod top;
mod user;

#[poise::command(
  prefix_command,
  slash_command,
  subcommands("top::top", "app::app", "link::link", "user::user", "update"),
  subcommand_required
)]
pub(crate) async fn steam(_: Ctx<'_>) -> R {
  Ok(())
}

#[poise::command(slash_command, owners_only)]
async fn update(ctx: Ctx<'_>) -> R {
  let state = ctx.data().read().await;
  let api = state.borrow::<SteamAPI>()?.clone();
  let pool = state.borrow::<PgPool>()?.clone();
  if let Err(err) = refresh_steam_data(&api, &pool).await {
    log::error!("An error occured while refreshing Steam data: {}", err);
  };
  Ok(())
}