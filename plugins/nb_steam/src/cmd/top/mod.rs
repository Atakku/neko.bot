// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

//TODO: REWRITE
use nb_poise::Ctx;
use nbf::R;

pub(crate) mod apps;
mod guilds;
mod users;

#[poise::command(
  prefix_command,
  slash_command,
  guild_only,
  subcommands("apps::apps", "guilds::guilds", "users::users"),
  subcommand_required
)]
pub(crate) async fn top(_: Ctx<'_>) -> R {
  Ok(())
}
