// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::env;

use nb_lifecycle::LifecycleFramework;
use nbf::{Framework, PluginLoader, R};

#[tokio::main]
async fn main() -> R {
  if !env::var("RUST_LOG").is_ok_and(|f| !f.is_empty()) {
    env::set_var("RUST_LOG", "warn");
  }
  pretty_env_logger::init();
  let mut fw = Framework::new();
  fw.init_plugin(nb_steam::SteamPlugin)?;

  // WIP
  fw.init_plugin(nb_anilist::AnilistPlugin)?;
  fw.init_plugin(nb_vrchat::VRCPlugin)?;

  fw.run().await?;
  Ok(())
}
