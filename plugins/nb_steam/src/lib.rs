// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, PluginLoader, R};

pub mod schema;

pub struct SteamPlugin;

impl Plugin for SteamPlugin {
  fn init(self, fw: &mut Framework) -> R {
    // Technically not needed as we already depend on DiscordPlugin
    fw.require_plugin::<nb_poise::PoisePlugin>()?;
    fw.require_plugin::<nb_sqlx::SqlxPlugin>()?;
    fw.require_plugin::<nb_discord::DiscordPlugin>()?;

    // Not 100% sure about priority
    fw.require_plugin::<nb_fluent::FluentPlugin>()?;
    Ok(())
  }
}
