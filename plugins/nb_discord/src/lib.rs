// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nb_framework::{Plugin, Framework, Res};

pub struct AnilistPlugin;

impl Plugin for AnilistPlugin {
  fn init(&self, _: &mut Framework) -> Res<&Self> {
    Ok(self)
  }
}
