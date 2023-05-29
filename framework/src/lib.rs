// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![feature(trait_alias)]

#[cfg(feature = "core")]
mod core;
#[cfg(feature = "core")]
pub use crate::core::*;

#[cfg(feature = "plugins")]
mod plugins;
#[cfg(feature = "plugins")]
pub use crate::plugins::*;

#[cfg(feature = "state")]
mod state;
#[cfg(feature = "state")]
pub use crate::state::*;

#[cfg(feature = "types")]
mod types;
#[cfg(feature = "types")]
pub use crate::types::*;
