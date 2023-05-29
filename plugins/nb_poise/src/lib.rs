// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::env;

use futures::future::join_all;
use nb_lifecycle::{ArcState, ArcStateHelper, LifecycleFramework, LifecyclePlugin};
use nbf::{Err, Framework, Plugin, PluginLoader, Res, R};
use poise::{
  serenity_prelude::{Context as SerenityContext, GatewayIntents},
  BoxFuture, Command, Context, Event, FrameworkContext, FrameworkOptions,
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

pub struct PoisePlugin {
  token: String,
}

impl Default for PoisePlugin {
  fn default() -> Self {
    Self {
      token: env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is not set"),
    }
  }
}

impl Plugin for PoisePlugin {
  fn init(self, fw: &mut Framework) -> R {
    fw.require_plugin::<LifecyclePlugin>()?;
    fw.state.put(Poise::builder().token(self.token.clone()));
    fw.state.put(Vec::<Cmd>::new());
    fw.state.put(Vec::<EventHandler>::new());
    fw.state.put(GatewayIntents::empty());
    fw.main_hook(|state| {
      Box::pin(async move {
        state
          .take::<PoiseBuilder>().await?
          .intents(state.take::<GatewayIntents>().await?)
          .options(FrameworkOptions {
            commands: state.take::<Vec<Cmd>>().await?,
            event_handler: |ctx, e, fctx, h| {
              Box::pin(async move {
                let ehs = h.read().await.borrow::<Vec<EventHandler>>()?.clone();
                join_all(
                  ehs
                    .iter()
                    .map(|eh| (eh)(ctx, e, fctx, h)),
                )
                .await;
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

pub trait PoiseFramework {
  fn add_command(&mut self, cmd: Cmd) -> Res<&mut Framework>;
  fn add_commands(&mut self, cmds: &mut Vec<Cmd>) -> Res<&mut Framework>;
  fn add_event_handler(&mut self, eh: EventHandler) -> Res<&mut Framework>;
  fn add_intents(&mut self, intents: GatewayIntents) -> Res<&mut Framework>;
}

impl PoiseFramework for Framework {
  fn add_command(&mut self, cmd: Cmd) -> Res<&mut Framework> {
    self.state.borrow_mut::<Vec<Cmd>>()?.push(cmd);
    Ok(self)
  }

  fn add_commands(&mut self, cmds: &mut Vec<Cmd>) -> Res<&mut Framework> {
    self.state.borrow_mut::<Vec<Cmd>>()?.append(cmds);
    Ok(self)
  }

  fn add_event_handler(&mut self, eh: EventHandler) -> Res<&mut Framework> {
    self.state.borrow_mut::<Vec<EventHandler>>()?.push(eh);
    Ok(self)
  }

  fn add_intents(&mut self, intents: GatewayIntents) -> Res<&mut Framework> {
    self.state.borrow_mut::<GatewayIntents>()?.insert(intents);
    Ok(self)
  }
}

/// Registers or unregisters application commands in this guild or globally
#[poise::command(prefix_command, hide_in_help, owners_only)]
#[cfg(debug_assertions)]
async fn register(ctx: Ctx<'_>) -> R {
  poise::samples::register_application_commands_buttons(ctx).await?;
  Ok(())
}
