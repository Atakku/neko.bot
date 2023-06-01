// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![feature(async_closure)]

#[cfg(feature = "schema")]
pub mod schema;

#[cfg(feature = "plugin")]
mod cmd;
#[cfg(feature = "plugin")]
mod query;
#[cfg(feature = "plugin")]
mod ui;

#[cfg(feature = "plugin")]
mod plugin;
#[cfg(feature = "plugin")]
pub use plugin::*;
