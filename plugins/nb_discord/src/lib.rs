// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, PluginLoader, R};

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
    Ok(())
  }
}
