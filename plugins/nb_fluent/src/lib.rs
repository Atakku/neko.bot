// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, R};

pub struct FluentPlugin;

impl Default for FluentPlugin {
  fn default() -> Self {
    Self {}
  }
}

impl Plugin for FluentPlugin {
  fn init(self, _: &mut Framework) -> R {
    Ok(())
  }
}
