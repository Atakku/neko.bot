// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use futures::future::join_all;
use nb_lifecycle::{LifecyclePlugin, LifecycleFramework, ArcStateHelper};
use nbf::{Framework, Plugin, R, PluginLoader};
use poise::{serenity_prelude::GatewayIntents, FrameworkOptions};

use crate::{EventHandler, Cmd, Ctx, Poise, PoiseBuilder, PoiseFramework};

pub struct PoisePlugin {
  token: String,
}

impl Default for PoisePlugin {
  fn default() -> Self {
    Self {
      token: std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is not set"),
    }
  }
}

impl Plugin for PoisePlugin {
  fn init(self, fw: &mut Framework) -> R {
    log::trace!("PoisePlugin::init()");
    fw.require_plugin::<LifecyclePlugin>()?;
    fw.state.put(Poise::builder().token(self.token.clone()));
    fw.state.put(Vec::<Cmd>::new());
    fw.state.put(Vec::<EventHandler>::new());
    fw.state.put(GatewayIntents::empty());
    fw.main_hook(|state| {
      Box::pin(async move {
        state
          .take::<PoiseBuilder>()
          .await?
          .intents(state.take::<GatewayIntents>().await?)
          .options(FrameworkOptions {
            commands: state.take::<Vec<Cmd>>().await?,
            event_handler: |ctx, e, fctx, h| {
              Box::pin(async move {
                let ehs = h.read().await.borrow::<Vec<EventHandler>>()?.clone();
                // TODO: Parallel?
                join_all(ehs.iter().map(|eh| (eh)(ctx, e, fctx, h))).await;
                Ok(())
              })
            },
            ..Default::default()
          })
          .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(state) }))
          .run()
          .await?;
        Ok(())
      })
    })?;
    // Only needed for development to register new commands
    #[cfg(debug_assertions)]
    {
      fw.add_intents(GatewayIntents::GUILD_MESSAGES)?;
      fw.add_command(register())?;
    }
    Ok(())
  }
}

/// Registers or unregisters application commands in this guild or globally
#[poise::command(prefix_command, hide_in_help, owners_only)]
#[cfg(debug_assertions)]
async fn register(ctx: Ctx<'_>) -> R {
  poise::samples::register_application_commands_buttons(ctx).await?;
  Ok(())
}