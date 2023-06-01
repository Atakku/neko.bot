// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, R};

pub struct AnilistPlugin;

impl Plugin for AnilistPlugin {
  fn init(self, _: &mut Framework) -> R {
    Ok(())
  }
}
