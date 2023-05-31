// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, R};

use crate::{FluentResources, LOCALES};

pub struct FluentPlugin;

impl Default for FluentPlugin {
  fn default() -> Self {
    Self {}
  }
}

impl Plugin for FluentPlugin {
  fn init(self, fw: &mut Framework) -> R {
    log::trace!("FluentPlugin::init()");
    let mut res = FluentResources::new();
    for locale in LOCALES {
      res.insert(locale.into(), vec![]);
    }
    fw.state.put(res);
    Ok(())
  }
}
