// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![feature(async_fn_in_trait)]

use futures::future::{join_all, LocalBoxFuture, BoxFuture};
use futures_locks::RwLock;
use nbf::{Framework, Res, R, SharedData, State, Plugin};

pub type HR = LocalBoxFuture<'static, R>;
pub type Hook = fn(ArcState) -> HR;

pub type MHR = BoxFuture<'static, R>;
pub type MainHook = fn(ArcState) -> MHR;

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

pub struct LifecyclePlugin;

impl Default for LifecyclePlugin {
  fn default() -> Self {
    Self {}
  }
}

impl Plugin for LifecyclePlugin {
  fn init(self, fw: &mut Framework) -> R {
    log::trace!("LifecyclePlugin::init()");
    fw.state.put(LifecycleHooks::default());
    Ok(())
  }
}

pub type ArcState = RwLock<State>;

pub trait ArcStateHelper {
  async fn put<T>(&self, t: T) -> R
  where T: SharedData;
  async fn has<T>(&self) -> Res<bool>
  where T: SharedData;
  async fn take<T>(&self) -> Res<T>
  where T: SharedData;
}

impl ArcStateHelper for ArcState {
  async fn put<T>(&self, t: T) -> R
  where T: SharedData {
    Ok(self.write().await.put(t))
  }

  async fn has<T>(&self) -> Res<bool>
  where T: SharedData {
    Ok(self.read().await.has::<T>())
  }

  async fn take<T>(&self) -> Res<T>
  where T: SharedData {
    self.write().await.take::<T>()
  }
}
