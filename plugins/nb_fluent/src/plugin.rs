// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::collections::HashMap;

use nb_lifecycle::LifecycleFramework;
use nbf::{Framework, Plugin, R};

use crate::{FluentBundle, FluentResources};

pub struct FluentPlugin;

impl Default for FluentPlugin {
  fn default() -> Self {
    Self {}
  }
}

impl Plugin for FluentPlugin {
  fn init(self, fw: &mut Framework) -> R {
    log::trace!("FluentPlugin::init()");
    fw.state.put(FluentResources::new());
    fw.pre_hook(|state| {
      Box::pin(async move {
        let mut state_mut = state.write().await;
        let raw_frs = state_mut.take::<FluentResources>()?;
        let mut fbs = HashMap::new();
        for (loc, frs) in raw_frs {
          let mut fb = FluentBundle::new_concurrent(vec![loc.parse()?]);
          for fr in frs {
            fb.add_resource(fr)
              .map_err(|e| format!("failed to add resource to bundle: {:?}", e))?;
          }
          fbs.insert(loc, fb);
        }
        state_mut.put(fbs);
        Ok(())
      })
    })?;
    #[cfg(feature = "poise")]
    fw.pre_hook(crate::poise::localize_commands)?;
    Ok(())
  }
}
