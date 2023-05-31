// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, R};

pub struct SvgUiPlugin;

impl Default for SvgUiPlugin {
  fn default() -> Self {
    Self {}
  }
}

impl Plugin for SvgUiPlugin {
  fn init(self, _: &mut Framework) -> R {
    Ok(())
  }
}
