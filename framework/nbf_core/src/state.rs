// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::{any::{TypeId, Any}, collections::HashMap};

/// Shared state

use crate::Res;
pub trait SharedData = Any + Send + Sync;
pub struct State {
  data: HashMap<TypeId, Box<dyn SharedData>>,
}

#[allow(dead_code)]
impl State {
  pub fn new() -> State {
    State {
      data: HashMap::new(),
    }
  }

  pub fn put<T>(&mut self, t: T)
  where T: SharedData {
    self.data.insert(TypeId::of::<T>(), Box::new(t));
  }

  pub fn has<T>(&self) -> bool
  where T: SharedData {
    self.data.get(&TypeId::of::<T>()).is_some()
  }

  pub fn try_get<T>(&self) -> Option<&T>
  where T: SharedData {
    self
      .data
      .get(&TypeId::of::<T>())
      .and_then(|b| b.downcast_ref::<T>())
  }

  pub fn get<T>(&self) -> Res<&T>
  where T: SharedData {
    Ok(
      self
        .try_get()
        .ok_or("Required type is not present in State container")?,
    )
  }

  pub fn get_or_default<T>(&mut self) -> Res<&T>
  where T: SharedData + Default {
    if !self.has::<T>() {
      self.put(T::default());
    }
    self.get()
  }

  pub fn try_get_mut<T>(&mut self) -> Option<&mut T>
  where T: SharedData {
    self
      .data
      .get_mut(&TypeId::of::<T>())
      .and_then(|b| b.downcast_mut::<T>())
  }

  pub fn get_mut<T>(&mut self) -> Res<&mut T>
  where T: SharedData {
    Ok(
      self
        .try_get_mut()
        .ok_or("Required type is not present in State container")?,
    )
  }

  pub fn get_mut_or_default<T>(&mut self) -> Res<&mut T>
  where T: SharedData + Default {
    if !self.has::<T>() {
      self.put(T::default());
    }
    self.get_mut()
  }
}
