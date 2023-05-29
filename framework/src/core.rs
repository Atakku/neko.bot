// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use crate::State;

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
