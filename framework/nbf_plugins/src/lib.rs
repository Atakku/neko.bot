// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![feature(async_fn_in_trait)]

use std::any::TypeId;

use nbf_core::{Framework, Res};

pub type PluginIndex = Vec<TypeId>;

pub trait Plugin {
  fn init(&self, fw: &mut Framework) -> Res<&Self>;
}
pub trait AsyncPlugin {
  async fn init(&self, fw: &mut Framework) -> Res<&Self>;
}

pub trait PluginLoader {
  fn has_plugin<T>(&mut self) -> Res<bool>
  where T: 'static;
  fn init_plugin<T>(&mut self, plugin: T) -> Res<&mut Self>
  where T: Plugin + Send + Sync + 'static;
  fn require_plugin<T>(&mut self) -> Res<&mut Self>
  where T: Plugin + Send + Sync + 'static + Default;
  async fn init_async_plugin<T>(&mut self, plugin: T) -> Res<&mut Self>
  where T: AsyncPlugin + Send + Sync + 'static;
  async fn require_async_plugin<T>(&mut self) -> Res<&mut Self>
  where T: AsyncPlugin + Send + Sync + 'static + Default;
}

impl PluginLoader for Framework {
  fn has_plugin<T>(&mut self) -> Res<bool>
  where T: 'static {
    Ok(
      self
        .state
        .get_or_default::<PluginIndex>()?
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

  async fn init_async_plugin<T>(&mut self, plugin: T) -> Res<&mut Self>
  where T: AsyncPlugin + Send + Sync + 'static {
    plugin.init(self).await?;
    self
      .state
      .get_mut_or_default::<PluginIndex>()?
      .push(TypeId::of::<T>());
    Ok(self)
  }

  async fn require_async_plugin<T>(&mut self) -> Res<&mut Self>
  where T: AsyncPlugin + Send + Sync + 'static + Default {
    if !self.has_plugin::<T>()? {
      self.init_async_plugin(T::default()).await?;
    }
    Ok(self)
  }
}
