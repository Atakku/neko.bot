// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Framework, Plugin, R};
use sqlx::{Database, pool::PoolOptions};

pub struct SqlxPlugin<T> where T: Database {
  db_url: String,
  opts: PoolOptions<T>,
}

impl<T> Default for SqlxPlugin<T> where T: Database  {
  fn default() -> Self {
    Self {
      db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL is not set"),
      opts: PoolOptions::<T>::new()
    }
  }
}

impl<T: sqlx::Database> Plugin for SqlxPlugin<T> {
  fn init(self, fw: &mut Framework) -> R {
    log::trace!("SqlxPlugin::init()");
    fw.state
      .put(self.opts.connect_lazy(&self.db_url)?);
    Ok(())
  }
}
