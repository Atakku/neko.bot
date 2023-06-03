// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, R};

pub struct SysMonPlugin;

impl Plugin for SysMonPlugin {
  fn init(self, _: &mut Framework) -> R {
    log::trace!("SysMonPlugin::init()");
    Ok(())
  }
}
