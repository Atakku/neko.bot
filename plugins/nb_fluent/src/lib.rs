// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::collections::HashMap;

use fluent::{bundle::FluentBundle as GenericFluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;

pub type FluentResources = HashMap<String, Vec<FluentResource>>;
pub type FluentBundle = GenericFluentBundle<FluentResource, IntlLangMemoizer>;
pub type FluentBundles = HashMap<String, FluentBundle>;

pub(crate) const LOCALES: [&str; 31] = [
  "id", "da", "de", "en-GB", "en-US", "es-ES", "fr", "hr", "it", "lt", "hu", "nl", "no", "pl",
  "pt-BR", "ro", "fi", "sv-SE", "vi", "tr", "cs", "el", "bg", "ru", "uk", "hi", "th", "zh-CN",
  "ja", "zh-TW", "ko",
];

mod framework;
pub use framework::*;
mod plugin;
pub use plugin::*;
