// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nbf::{Res, Framework};
use poise::serenity_prelude::GatewayIntents;

use crate::{EventHandler, Cmd};

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