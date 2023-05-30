// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

//TODO: REWRITE
use nb_poise::Ctx;
use nbf::R;

// Information about a user
#[poise::command(prefix_command, slash_command)]
pub(crate) async fn user(_: Ctx<'_>) -> R {
  Ok(())
}
