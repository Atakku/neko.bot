// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![feature(trait_alias)]

use std::error::Error;

use state::State;

/// Generic error & result aliases
pub type Err = Box<dyn Error + Send + Sync>;
pub type Res<T> = Result<T, Err>;
pub type R = Res<()>;

mod state;
pub use state::*;

/// Framework core
pub struct Framework {
  pub state: State,
}

impl Framework {
  pub fn new() -> Self {
    Self {
      state: State::new(),
    }
  }
}
