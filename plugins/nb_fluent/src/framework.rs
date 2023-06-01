// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::any::type_name;

use fluent::FluentResource;
use nbf::{Framework, Res};
use rust_embed::RustEmbed;

use crate::FluentResources;

pub trait FluentFramework {
  fn add_fluent_resources<T>(&mut self) -> Res<&mut Framework>
  where T: RustEmbed;
}

impl FluentFramework for Framework {
  fn add_fluent_resources<T>(&mut self) -> Res<&mut Framework>
  where T: RustEmbed {
    log::trace!("FluentFramework::add_fluent_resources()");
    let res = self.state.borrow_mut::<FluentResources>()?;
    for locale in T::iter()
      .filter(|n| n.ends_with(".ftl"))
      .map(|n| n.replace(".ftl", ""))
    {
      if !res.contains_key(&locale) {
        res.insert(locale.clone(), vec![]);
      }
      if let Some(file) = T::get(format!("{locale}.ftl").as_str()) {
        res
          .get_mut(&locale)
          .ok_or("missing {locale} in FluentResources")?
          .push(
            FluentResource::try_new(String::from_utf8(file.data.to_vec())?)
              .map_err(|(_, e)| format!("failed to parse {:?}", e))?,
          );
      } else {
        log::warn!("{} is missing {locale}.ftl", type_name::<T>())
      }
    }
    Ok(self)
  }
}
