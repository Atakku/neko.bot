// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

//TODO: REWRITE
use nb_poise::Ctx;
use nbf::R;

mod app;
mod link;
pub(crate) mod top;
mod user;

#[poise::command(
  prefix_command,
  slash_command,
  subcommands("top::top", "app::app", "link::link", "user::user", "crate::update"),
  subcommand_required
)]
pub(crate) async fn steam(_: Ctx<'_>) -> R {
  Ok(())
}
