// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::collections::HashMap;

use fluent::{bundle::FluentBundle as GenericFluentBundle, FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;

pub type FluentResources = HashMap<String, Vec<FluentResource>>;
pub type FluentBundle = GenericFluentBundle<FluentResource, IntlLangMemoizer>;
pub type FluentBundles = HashMap<String, FluentBundle>;

#[cfg(feature = "framework")]
mod framework;
#[cfg(feature = "framework")]
pub use framework::*;

#[cfg(feature = "plugin")]
mod plugin;
#[cfg(feature = "plugin")]
pub use plugin::*;

#[cfg(feature = "poise")]
mod poise;

// TODO: Move into poise or use somewhere else
#[allow(dead_code)]
pub(crate) fn localize<'a>(
  fb: &FluentBundle,
  id: &str,
  attr: Option<&str>,
  args: Option<&FluentArgs<'_>>,
) -> Option<String> {
  let message = fb.get_message(id)?;
  let pattern = match attr {
    Some(attribute) => message.get_attribute(attribute)?.value(),
    None => message.value()?,
  };
  Some(fb.format_pattern(pattern, args, &mut vec![]).to_string())
}
