// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, R};

pub struct VRCPlugin;

impl Plugin for VRCPlugin {
  fn init(self, _: &mut Framework) -> R {
    log::trace!("VRCPlugin::init()");
    Ok(())
  }
}
