// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, R};

use crate::LifecycleHooks;

pub struct LifecyclePlugin;

impl Default for LifecyclePlugin {
  fn default() -> Self {
    Self {}
  }
}

impl Plugin for LifecyclePlugin {
  fn init(self, fw: &mut Framework) -> R {
    fw.state.put(LifecycleHooks::default());
    Ok(())
  }
}