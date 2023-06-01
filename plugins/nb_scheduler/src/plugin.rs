// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, R};

pub struct SchedulerPlugin;

impl Plugin for SchedulerPlugin {
  fn init(self, _: &mut Framework) -> R {
    log::trace!("SchedulerPlugin::init()");
    Ok(())
  }
}
