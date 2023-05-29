// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![feature(async_fn_in_trait)]

use std::sync::{Arc, RwLock};

use futures::future::{join_all, LocalBoxFuture};
use nbf::{Framework, Plugin, Res, SharedData, State, R};

pub trait ArcState {
  fn put<T>(&self, t: T) -> R
  where T: SharedData;
  fn has<T>(&self) -> Res<bool>
  where T: SharedData;
  fn take<T>(&self) -> Res<T>
  where T: SharedData;
}

impl ArcState for Arc<RwLock<State>> {
  fn put<T>(&self, t: T) -> R
  where T: SharedData {
    Ok(self.write().map_err(|_| "RwLock is poisoned")?.put(t))
  }

  fn has<T>(&self) -> Res<bool>
  where T: SharedData {
    Ok(self.read().map_err(|_| "RwLock is poisoned")?.has::<T>())
  }

  fn take<T>(&self) -> Res<T>
  where T: SharedData {
    self.write().map_err(|_| "RwLock is poisoned")?.take::<T>()
  }
}

pub type Hook = fn(Arc<RwLock<State>>) -> LocalBoxFuture<'static, R>;

pub struct LifecycleHooks {
  pre: Vec<Hook>,
  main: Vec<Hook>,
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

pub struct LifecyclePlugin;

impl Default for LifecyclePlugin {
  fn default() -> Self {
    Self {}
  }
}

impl Plugin for LifecyclePlugin {
  fn init(self, fw: &mut Framework) -> R {
    fw.state.put(LifecycleHooks::default());
    Ok(())
  }
}

pub trait LifecycleFramework {
  fn pre_hook(&mut self, hook: Hook) -> Res<&mut Framework>;
  fn main_hook(&mut self, hook: Hook) -> Res<&mut Framework>;
  fn post_hook(&mut self, hook: Hook) -> Res<&mut Framework>;
  async fn run(self) -> R;
}

impl LifecycleFramework for Framework {
  fn pre_hook(&mut self, hook: Hook) -> Res<&mut Framework> {
    self.state.borrow_mut::<LifecycleHooks>()?.pre.push(hook);
    Ok(self)
  }
  fn main_hook(&mut self, hook: Hook) -> Res<&mut Framework> {
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
    let state = Arc::new(RwLock::new(state));
    run_hooks(hooks.pre, &state).await?;
    run_hooks(hooks.main, &state).await?;
    run_hooks(hooks.post, &state).await?;
    Ok(())
  }
}

async fn run_hooks(hooks: Vec<Hook>, state: &Arc<RwLock<State>>) -> R {
  for res in join_all(hooks.into_iter().map(|h| (h)(state.clone()))).await {
    res?
  }
  Ok(())
}
