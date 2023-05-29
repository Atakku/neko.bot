// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, R};

pub struct SchedulerPlugin;

impl Default for SchedulerPlugin {
  fn default() -> Self {
    Self {}
  }
}

impl Plugin for SchedulerPlugin {
  fn init(self, _: &mut Framework) -> R {
    Ok(())
  }
}
