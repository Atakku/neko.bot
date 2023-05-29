// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::env;

use nbf::{Framework, Plugin, Res, State, R};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct SqlxPlugin {
  db_url: String,
}

impl Default for SqlxPlugin {
  fn default() -> Self {
    Self {
      db_url: env::var("DATABASE_URL").expect("DATABASE_URL is not set"),
    }
  }
}

impl Plugin for SqlxPlugin {
  fn init(self, fw: &mut Framework) -> R {
    fw.state
      .put(PgPoolOptions::new().connect_lazy(&self.db_url)?);
    Ok(())
  }
}

pub trait SqlxState {
  fn db(&self) -> Res<&PgPool>;
}
impl SqlxState for State {
  fn db(&self) -> Res<&PgPool> {
    self.borrow()
  }
}
