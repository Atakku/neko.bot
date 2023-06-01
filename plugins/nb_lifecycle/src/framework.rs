// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use futures::future::join_all;
use futures_locks::RwLock;
use nbf::{Framework, Res, R};

use crate::{Hook, MainHook, ArcState};

pub struct LifecycleHooks {
  pre: Vec<Hook>,
  main: Vec<MainHook>,
  post: Vec<Hook>,
}

impl Default for LifecycleHooks {
  fn default() -> Self {
    Self {
      pre: Vec::new(),
      main: Vec::new(),
      post: Vec::new(),
    }
  }
}

pub trait LifecycleFramework {
  fn pre_hook(&mut self, hook: Hook) -> Res<&mut Framework>;
  fn main_hook(&mut self, hook: MainHook) -> Res<&mut Framework>;
  fn post_hook(&mut self, hook: Hook) -> Res<&mut Framework>;
  async fn run(self) -> R;
}

impl LifecycleFramework for Framework {
  fn pre_hook(&mut self, hook: Hook) -> Res<&mut Framework> {
    self.state.borrow_mut::<LifecycleHooks>()?.pre.push(hook);
    Ok(self)
  }
  fn main_hook(&mut self, hook: MainHook) -> Res<&mut Framework> {
    self.state.borrow_mut::<LifecycleHooks>()?.main.push(hook);
    Ok(self)
  }
  fn post_hook(&mut self, hook: Hook) -> Res<&mut Framework> {
    self.state.borrow_mut::<LifecycleHooks>()?.post.push(hook);
    Ok(self)
  }

  async fn run(self) -> R {
    let mut state = self.state;
    let hooks = state.take::<LifecycleHooks>()?;
    let state = RwLock::new(state);
    run_sequential(hooks.pre, &state).await?;
    run_parallel(hooks.main, &state).await?;
    run_sequential(hooks.post, &state).await?;
    Ok(())
  }
}

async fn run_sequential(hooks: Vec<Hook>, state: &ArcState) -> R {
  for hook in hooks {
    (hook)(state.clone()).await?;
  }
  Ok(())
}

async fn run_parallel(hooks: Vec<MainHook>, state: &ArcState) -> R {
  for res in join_all(hooks.into_iter().map(|h| tokio::spawn((h)(state.clone())))).await {
    res??
  }
  Ok(())
}