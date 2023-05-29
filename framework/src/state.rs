// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::{
  any::{Any, TypeId},
  collections::HashMap,
  hash::{BuildHasherDefault, Hasher},
};

use crate::Res;

pub trait SharedData = Any + Send + Sync;

// Code taken from https://github.com/gotham-rs/gotham/blob/main/gotham/src/state/mod.rs
#[derive(Default)]
struct IdHasher(u64);
impl Hasher for IdHasher {
  fn write(&mut self, _: &[u8]) {
    unreachable!("TypeId calls write_u64");
  }

  #[inline]
  fn write_u64(&mut self, id: u64) {
    self.0 = id;
  }

  #[inline]
  fn finish(&self) -> u64 {
    self.0
  }
}

pub struct State {
  data: HashMap<TypeId, Box<dyn SharedData>, BuildHasherDefault<IdHasher>>,
}

#[allow(dead_code)]
impl State {
  pub fn new() -> State {
    State {
      data: HashMap::default(),
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

  pub fn try_borrow<T>(&self) -> Option<&T>
  where T: SharedData {
    self
      .data
      .get(&TypeId::of::<T>())
      .and_then(|b| b.downcast_ref::<T>())
  }

  pub fn borrow<T>(&self) -> Res<&T>
  where T: SharedData {
    Ok(
      self
        .try_borrow()
        .ok_or("Required type is not present in State container")?,
    )
  }

  pub fn borrow_or_default<T>(&mut self) -> Res<&T>
  where T: SharedData + Default {
    if !self.has::<T>() {
      self.put(T::default());
    }
    self.borrow()
  }

  pub fn try_borrow_mut<T>(&mut self) -> Option<&mut T>
  where T: SharedData {
    self
      .data
      .get_mut(&TypeId::of::<T>())
      .and_then(|b| b.downcast_mut::<T>())
  }

  pub fn borrow_mut<T>(&mut self) -> Res<&mut T>
  where T: SharedData {
    Ok(
      self
        .try_borrow_mut()
        .ok_or("Required type is not present in State container")?,
    )
  }

  pub fn get_mut_or_default<T>(&mut self) -> Res<&mut T>
  where T: SharedData + Default {
    if !self.has::<T>() {
      self.put(T::default());
    }
    self.borrow_mut()
  }

  pub fn try_take<T>(&mut self) -> Option<T>
  where T: SharedData {
    self
      .data
      .remove(&TypeId::of::<T>())
      .and_then(|b| b.downcast::<T>().ok())
      .map(|b| *b)
  }

  pub fn take<T>(&mut self) -> Res<T>
  where T: SharedData {
    Ok(
      self
        .try_take()
        .ok_or("required type is not present in State container")?,
    )
  }
}
