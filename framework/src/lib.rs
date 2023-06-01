// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![feature(trait_alias)]

use std::error::Error;

pub type Err = Box<dyn Error + Send + Sync>;
pub type Res<T> = Result<T, Err>;
pub type R = Res<()>;

#[cfg(feature = "framework")]
mod framework;
#[cfg(feature = "framework")]
pub use crate::framework::*;

#[cfg(feature = "plugin")]
mod plugin;
#[cfg(feature = "plugin")]
pub use crate::plugin::*;

#[cfg(feature = "state")]
mod state;
#[cfg(feature = "state")]
pub use crate::state::*;
