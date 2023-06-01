// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nb_lifecycle::ArcState;
use nbf::{Err, R};
use poise::{
  serenity_prelude::Context as SerenityContext, BoxFuture, Command, Context, Event,
  FrameworkContext,
};

pub type Poise = poise::Framework<ArcState, Err>;
pub type PoiseBuilder = poise::FrameworkBuilder<ArcState, Err>;
pub type Ctx<'a> = Context<'a, ArcState, Err>;

pub type EventHandler = for<'a> fn(
  &'a SerenityContext,
  &'a Event<'a>,
  FrameworkContext<'a, ArcState, Err>,
  &'a ArcState,
) -> BoxFuture<'a, R>;
pub type Cmd = Command<ArcState, Err>;

#[cfg(feature = "framework")]
mod framework;
#[cfg(feature = "framework")]
pub use framework::*;

#[cfg(feature = "plugin")]
mod plugin;
#[cfg(feature = "plugin")]
pub use plugin::*;
