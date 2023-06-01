// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![feature(async_fn_in_trait)]

use futures::future::{BoxFuture, LocalBoxFuture};
use futures_locks::RwLock;
use nbf::{Res, SharedData, State, R};

pub type ArcState = RwLock<State>;

pub type HR = LocalBoxFuture<'static, R>;
pub type Hook = fn(ArcState) -> HR;

pub type MHR = BoxFuture<'static, R>;
pub type MainHook = fn(ArcState) -> MHR;

#[cfg(feature = "framework")]
mod framework;
#[cfg(feature = "framework")]
pub use framework::*;

#[cfg(feature = "plugin")]
mod plugin;
#[cfg(feature = "plugin")]
pub use plugin::*;


pub trait ArcStateHelper {
  async fn put<T>(&self, t: T) -> R
  where T: SharedData;
  async fn has<T>(&self) -> Res<bool>
  where T: SharedData;
  async fn take<T>(&self) -> Res<T>
  where T: SharedData;
}

impl ArcStateHelper for ArcState {
  async fn put<T>(&self, t: T) -> R
  where T: SharedData {
    Ok(self.write().await.put(t))
  }

  async fn has<T>(&self) -> Res<bool>
  where T: SharedData {
    Ok(self.read().await.has::<T>())
  }

  async fn take<T>(&self) -> Res<T>
  where T: SharedData {
    self.write().await.take::<T>()
  }
}
