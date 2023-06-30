// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use nb_lifecycle::LifecycleFramework;
use nbf::{Framework, PluginLoader, R};
use poise::{Event, serenity_prelude::{ChannelId, Colour, User}};
use sqlx::PgPool;
use nb_poise::PoiseFramework;

#[tokio::main]
async fn main() -> R {
  if !std::env::var("RUST_LOG").is_ok_and(|f| !f.is_empty()) {
    std::env::set_var("RUST_LOG", "warn");
  }
  pretty_env_logger::init();
  let mut fw = Framework::new();
  fw.init_plugin(nb_steam::SteamPlugin::default())?;

  fw.add_event_handler(|ctx, e, _fctx, s| {
    Box::pin(async move {
      let db = s.read().await.borrow::<PgPool>()?.clone();
      match e {
        // User joined
        Event::GuildMemberAddition { new_member } => {
          ChannelId(1038997543515865148).send_message(ctx, |m| {
            m.embed( |e| {
              let user = &new_member.user;
              e.author( |a| {
                a.icon_url(get_avatar(&user));
                a.name(if user.discriminator != 0 {
                  format!("{}#{:0>4}", user.name, user.discriminator)
                } else {
                  user.name.clone()
                });
                a.url(format!("https://discord.com/users/{}", user.id))
              });
              e.colour(Colour::from_rgb(139, 195, 74));
              e.description(format!("Welcome <@{}> to the server!", user.id))
            })
          }).await?;
        },
        Event::GuildMemberRemoval { guild_id, user, member_data_if_available } => {
          ChannelId(1038997543515865148).send_message(ctx, |m| {
            m.embed( |e| {
              e.author( |a| {
                a.icon_url(get_avatar(&user));
                a.name(if user.discriminator != 0 {
                  format!("{}#{:0>4}", user.name, user.discriminator)
                } else {
                  user.name.clone()
                });
                a.url(format!("https://discord.com/users/{}", user.id))
              });
              e.colour(Colour::from_rgb(244, 67, 54));
              e.description(format!("<@{}> has left the server!", user.id))
            })
          }).await?;
        },
        _ => {}
      }
      Ok(())
    })
  })?;

  // WIP
  fw.init_plugin(nb_anilist::AnilistPlugin)?;
  fw.init_plugin(nb_vrchat::VRCPlugin)?;

  fw.run().await?;
  Ok(())
}


fn get_avatar(u: &User) -> String {
  if let Some(avatar) = u.avatar_url() {
    return avatar;
  }
  u.default_avatar_url()
}