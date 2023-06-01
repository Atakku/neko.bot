// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

//TODO: REWRITE
use std::path::Path;

use nb_poise::Ctx;
use nbf::R;
use poise::{
  futures_util::future::join_all,
  serenity_prelude::{
    AttachmentType, ButtonStyle, CollectComponentInteraction, CreateActionRow, EditMessage,
    InteractionResponseType, ReactionType, UserId,
  },
};
use sqlx::{FromRow, PgPool};

use crate::{
  query::{paged_query, top_query_builder, TopQueryArgs},
  ui::{render_svg, test_path, TopUI},
};

// Top apps
#[poise::command(prefix_command, slash_command, guild_only)]
pub(crate) async fn apps(ctx: Ctx<'_>, global: Option<bool>, user: Option<UserId>) -> R {
  let Some(guild_id) = ctx.guild_id() else { return Ok(())};

  let mut msg = ctx
    .send(|b| {
      b.attachment(AttachmentType::Path(Path::new("empty.png")))
        .components(|b| {
          b.create_action_row(|b| pagination_buttons(b, 0, 0, false, "pg_next".into()))
        })
    })
    .await?
    .into_message()
    .await?;
  let qb = top_query_builder(TopQueryArgs::TopApps {
    guild_id: guild_id.0 as i64,
    global: global.unwrap_or(false),
    user_id: user.and_then(|u| Some(u.0 as i64)),
  })
  .to_owned();
  let pool = ctx.data().read().await.borrow::<PgPool>()?.clone();

  let render_page = async move |page| {
    let data = paged_query::<TopAppsRow>(qb, &pool, 10, page).await?;
    join_all(
      data
        .iter()
        .map(|i| test_path(i.id as u32, "library_600x900")),
    )
    .await;
    //data..await?;
    render_svg(TopUI {
      data: data.into_iter().collect::<Vec<TopAppsRow>>(),
      page,
      pages: 100,
    })
  };
  let mut page = 0;
  let mut pages = 100;

  let firstpage = render_page.clone()(page).await?;
  msg
    .edit(ctx, |b| {
      remove_attachments(b);
      b.attachment(AttachmentType::Bytes {
        data: firstpage.into(),
        filename: format!("page_{}.png", page),
      })
      .components(|b| b.create_action_row(|b| pagination_buttons(b, page, pages, false, "".into())))
    })
    .await?;

  let mut id = msg.id.0;
  while let Some(press) = CollectComponentInteraction::new(ctx)
    .message_id(msg.id)
    .timeout(std::time::Duration::from_secs(300))
    .await
  {
    match press.data.custom_id.as_str() {
      "pg_prev" => page -= 1,
      "pg_next" => page += 1,
      _ => {}
    }

    press
      .create_interaction_response(ctx, |f| {
        f.kind(InteractionResponseType::UpdateMessage)
          .interaction_response_data(|b| {
            b.components(|b| {
              b.create_action_row(|b| {
                pagination_buttons(b, page, pages, true, press.data.custom_id.clone())
              })
            })
          })
      })
      .await?;

    let pageee = render_page.clone()(page).await.unwrap();

    let mut msg = press.get_interaction_response(ctx).await?;
    msg
      .edit(ctx, |b| {
        remove_attachments(b);
        b.attachment(AttachmentType::Bytes {
          data: pageee.into(),
          filename: format!("page_{}.png", page),
        })
        .components(|b| {
          b.create_action_row(|b| {
            pagination_buttons(b, page, pages, false, press.data.custom_id.clone())
          })
        })
      })
      .await?;

    id = msg.id.0;
  }
  //let newid = ctx.http().get_message(msg.channel_id.0, msg.id.0).await?.attachments.get(0).unwrap().id;
  ctx
    .http()
    .get_message(msg.channel_id.0, id)
    .await?
    .edit(ctx, |m| {
      m.components(|b| b.create_action_row(|b| pagination_buttons(b, page, pages, true, "".into())))
    })
    .await?;
  Ok(())
}

#[derive(FromRow, Clone)]
#[allow(dead_code)]
pub struct TopAppsRow {
  pub id: i32,
  pub name: String,
  pub mins_sum: i64,
  pub row_num: i64,
}

fn remove_attachments(b: &mut EditMessage) {
  b.0
    .entry("attachments")
    .or_default()
    .as_array_mut()
    .expect("Attachments must be an array")
    .clear();
}
fn pagination_buttons(
  b: &mut CreateActionRow,
  page: u64,
  pages: u64,
  loading: bool,
  event: String,
) -> &mut CreateActionRow {
  let l = ReactionType::Custom {
    animated: true,
    id: poise::serenity_prelude::EmojiId(1110725977069326346),
    name: None,
  };
  b.create_button(|b| {
    if event == "pg_prev" && loading {
      b.emoji(l.clone())
    } else {
      b.label("<")
    }
    .custom_id(format!("pg_prev"))
    .style(ButtonStyle::Secondary)
    .disabled(loading || page == 0)
  });
  b.create_button(|b| {
    b.custom_id(format!("pg_disp"))
      .style(ButtonStyle::Secondary)
      .label(format!("{}/{pages}", page + 1))
      .disabled(true)
  });
  b.create_button(|b| {
    if event == "pg_next" && loading {
      b.emoji(l)
    } else {
      b.label(">")
    }
    .custom_id(format!("pg_next"))
    .style(ButtonStyle::Secondary)
    .disabled(loading || page + 1 == pages)
  });

  b
}
