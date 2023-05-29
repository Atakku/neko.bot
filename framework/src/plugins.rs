// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::any::TypeId;

use crate::core::Framework;
use crate::types::{Res, R};

pub type PluginIndex = Vec<TypeId>;

pub trait Plugin {
  fn init(self, fw: &mut Framework) -> R;
}

pub trait PluginLoader {
  fn has_plugin<T>(&mut self) -> Res<bool>
  where T: 'static;
  fn init_plugin<T>(&mut self, plugin: T) -> Res<&mut Self>
  where T: Plugin + Send + Sync + 'static;
  fn require_plugin<T>(&mut self) -> Res<&mut Self>
  where T: Plugin + Send + Sync + 'static + Default;
}

impl PluginLoader for Framework {
  fn has_plugin<T>(&mut self) -> Res<bool>
  where T: 'static {
    Ok(
      self
        .state
        .borrow_or_default::<PluginIndex>()?
        .contains(&TypeId::of::<T>()),
    )
  }

  fn init_plugin<T>(&mut self, plugin: T) -> Res<&mut Self>
  where T: Plugin + Send + Sync + 'static {
    plugin.init(self)?;
    self
      .state
      .get_mut_or_default::<PluginIndex>()?
      .push(TypeId::of::<T>());
    Ok(self)
  }

  fn require_plugin<T>(&mut self) -> Res<&mut Self>
  where T: Plugin + Send + Sync + 'static + Default {
    if !self.has_plugin::<T>()? {
      self.init_plugin(T::default())?;
    }
    Ok(self)
  }
}
