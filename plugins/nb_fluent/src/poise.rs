// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nb_lifecycle::{ArcState, HR};
use nb_poise::Cmd;

use crate::{localize, FluentBundle, FluentBundles};

pub(crate) const LOCS: [&str; 31] = [
  "id", "da", "de", "en-GB", "en-US", "es-ES", "fr", "hr", "it", "lt", "hu", "nl", "no", "pl",
  "pt-BR", "ro", "fi", "sv-SE", "vi", "tr", "cs", "el", "bg", "ru", "uk", "hi", "th", "zh-CN",
  "ja", "zh-TW", "ko",
];

pub(crate) fn localize_commands(state: ArcState) -> HR {
  Box::pin(async move {
    log::trace!("localize_commands()");
    let mut state_mut = state.write().await;
    let mut cmds = state_mut.take::<Vec<nb_poise::Cmd>>()?;
    let bundles = state_mut.borrow::<FluentBundles>()?;
    for (loc, fb) in bundles
      .into_iter()
      .filter(|(l, _)| LOCS.contains(&l.as_str()))
    {
      log::info!("Adding locale '{loc}' to {} commands", cmds.len());
      for cmd in &mut cmds {
        log::trace!("Adding locale '{loc}' to {}", cmd.name);
        crate::poise::apply_cmd_loc(cmd, loc, fb, None, false)
      }
    }
    if let Some(fb) = bundles.get("en-US") {
      log::trace!("Adding default locale 'en-US'");
      for cmd in &mut cmds {
        log::trace!("Adding locale 'en-US' to {}", cmd.name);
        crate::poise::apply_cmd_loc(cmd, "en-US", fb, None, true)
      }
    } else {
      log::warn!("Default locale 'en-US' was not found");
    }
    state_mut.put(cmds);
    Ok(())
  })
}

fn apply_cmd_loc(
  cmd: &mut Cmd,
  loc: &str,
  fb: &FluentBundle,
  parent_path: Option<&str>,
  default: bool,
) {
  let path = format!("{}_{}", parent_path.unwrap_or("cmd"), cmd.name);
  // Skip trying to localize group commands
  if !cmd.subcommand_required {
    if let Some(name) = try_cmd_loc(loc, fb, &path, None, true) {
      if default {
        cmd.name = name.into();
      } else {
        cmd.name_localizations.insert(loc.into(), name.into());
      }
    }
    if let Some(desc) = try_cmd_loc(loc, fb, &path, Some("desc"), false) {
      if default {
        cmd.description = Some(desc.into());
      } else {
        cmd
          .description_localizations
          .insert(loc.into(), desc.into());
      }
    }
    for prm in &mut cmd.parameters {
      let prm_path = format!("prm_{}", &prm.name);
      if let Some(name) = try_cmd_loc(loc, fb, &path, Some(&prm_path), true) {
        if default {
          log::error!("{} -> {}", prm.name, name);
          prm.name = name.into();
        } else {
          prm.name_localizations.insert(loc.into(), name.into());
        }
      }
      if let Some(desc) = try_cmd_loc(loc, fb, &path, Some(&format!("{prm_path}_desc")), false) {
        if default {
          prm.description = Some(desc.into());
        } else {
          prm
            .description_localizations
            .insert(loc.into(), desc.into());
        }
      }
      for cho in &mut prm.choices {
        let path = format!("cho_{}", &prm.name);
        if let Some(name) = try_cmd_loc(loc, fb, &path, Some(&cho.name), false) {
          if default {
            cho.name = name.into();
          } else {
            cho.localizations.insert(loc.into(), name.into());
          }
        }
      }
    }
  }
  for sub in &mut cmd.subcommands {
    apply_cmd_loc(sub, loc, fb, Some(&path), default);
  }
}

fn try_cmd_loc<'a>(
  locale: &str,
  fb: &FluentBundle,
  path: &str,
  attr: Option<&str>,
  lc: bool,
) -> Option<String> {
  log::trace!("try_cmd_loc()");
  let log_path = attr
    .and_then(|a| Some(format!("{path}.{a}")))
    .unwrap_or(path.into());
  if let Some(localized) = localize(fb, path, attr, None) {
    if !lc || localized.chars().all(char::is_lowercase) {
      log::info!("Returning from locale '{locale}' at path '{log_path}' value {localized}");
      return Some(localized);
    } else {
      log::error!("Locale '{locale}' contains uppercase characters in '{log_path}'")
    }
  } else {
    log::warn!("Locale '{locale}' is missing '{log_path}'")
  }
  return None;
}
