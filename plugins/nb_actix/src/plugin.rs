// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use actix_web::App;
use nbf::{Framework, Plugin, R};

pub struct ActixPlugin;

impl Plugin for ActixPlugin {
  fn init(self, fw: &mut Framework) -> R {
    log::trace!("ActixPlugin::init()");
    //fw.state.put("");
    
    Ok(())
  }
}
