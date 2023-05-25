// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nb_framework::{Plugin, Framework, Res};

pub struct SteamPlugin;

impl Plugin for SteamPlugin {
  fn init(&self, _: &mut Framework) -> Res<&Self> {
    Ok(self)
  }
}
