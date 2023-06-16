// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nb_lifecycle::LifecycleFramework;
use nbf::{Framework, Plugin, R};

pub struct HyperPlugin;

impl Plugin for HyperPlugin {
  fn init(self, fw: &mut Framework) -> R {
    log::trace!("HyperPlugin::init()");
    fw.main_hook(|state| {
      Box::pin(async move {
        
        Ok(())
      })
    })?;
    Ok(())
  }
}
