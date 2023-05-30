// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

//TODO: REWRITE
use itertools::Itertools;
use nb_poise::Ctx;
use nbf::{Err, R};
use nbl_steam_api::SteamAPI;
use poise::serenity_prelude::{
  ButtonStyle, CollectComponentInteraction, InteractionResponseType, UserId,
};
use sea_query::{Alias, Expr, Order, PostgresQueryBuilder, Query, UnionType};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, PgPool};

use crate::{query::paged_query, refresh_single_steam_user, schema::Accounts};
use nb_discord::schema::Members;

#[poise::command(
  prefix_command,
  slash_command,
  owners_only,
  subcommands("add", "missing")
)]
pub async fn link(_: Ctx<'_>) -> R {
  Ok(())
}

#[poise::command(prefix_command, slash_command, owners_only)]
async fn add(ctx: Ctx<'_>, u: UserId, pid: String) -> R {
  let (q, v) = Query::insert()
    .into_table(Accounts::Table)
    .columns([Accounts::id, Accounts::discord_id])
    .values([(pid.parse::<i64>()?).into(), u.0.into()])?
    .build_sqlx(PostgresQueryBuilder);

  let pool = ctx.data().read().await.borrow::<PgPool>()?.clone();
  let aaaa = sqlx::query_with(&q, v).execute(&pool).await;

  ctx
    .send(|b| {
      b.ephemeral(true).content(match aaaa {
        Ok(_) => format!("Added `{}` for {}", pid, u),
        Err(err) => format!("Failed, error: {}", err),
      })
    })
    .await?;

  let state = ctx.data().read().await;
  let api = state.borrow::<SteamAPI>()?.clone();
  let pool = state.borrow::<PgPool>()?.clone();
  refresh_single_steam_user(&api, &pool, pid.parse()?).await?;
  Ok(())
}

//#[poise::command(slash_command, owners_only)]
//async fn list(ctx: Ctx<'_>, m: Option<Member>) -> R {
//  ctx
//    .send(|b| b.ephemeral(true).content(format!("Listing {:?}", m)))
//    .await?;
//  Ok(())
//}

#[poise::command(prefix_command, slash_command, owners_only)]
async fn missing(
  ctx: Ctx<'_>,
  global: Option<bool>,
  escape: Option<bool>,
  extra: Option<bool>,
) -> R {
  let mut msg = ctx.send(|b| b.ephemeral(true).content("loading")).await?;
  let esc = if escape.unwrap_or(false) { "\\" } else { "" };
  let ext = if extra.unwrap_or(false) { 80 } else { 20 };
  let ec = if extra.unwrap_or(false) { 100 } else { 5 };
  let Some(guild_id) = ctx.guild_id() else { return Ok(())};
  let mut qb = Query::select()
    .expr_as(
      Expr::col((Members::Table, Members::user_id)),
      Alias::new("id"),
    )
    .from(Members::Table)
    .union(
      UnionType::Except,
      Query::select()
        .expr_as(
          Expr::col((Accounts::Table, Accounts::discord_id)),
          Alias::new("id"),
        )
        .from(Accounts::Table)
        .take(),
    )
    .distinct_on([Alias::new("id")])
    .order_by(Alias::new("id"), Order::Asc)
    .to_owned();
  if !global.unwrap_or(false) {
    qb.and_where(Expr::col((Members::Table, Members::guild_id)).eq(guild_id.0 as i64));
  }
  let render_page = async move |page| {
    let pool = ctx.data().read().await.borrow::<PgPool>()?.clone();
    let data = paged_query::<LinkRow>(qb, &pool, ext, page).await?;
    Ok::<_, Err>(
      data
        .into_iter()
        .map(|i| format!("{}<@{}>\n", esc, i.id))
        .chunks(ec)
        .into_iter()
        .map(|c| format!("{}========\n", c.collect::<String>()))
        .collect::<String>(),
    )
  };

  let ctx_id = ctx.id();
  let prev_button_id = format!("{}_prev", ctx.id());
  let next_button_id = format!("{}_next", ctx.id());
  let mut page = 0;
  let mut pages = 100;

  let firstpage = render_page.clone()(page).await?;
  msg
    .edit(ctx, |b| {
      b.content(firstpage).components(|b| {
        b.create_action_row(|b| {
          b.create_button(|b| {
            b.custom_id(&prev_button_id)
              .style(ButtonStyle::Secondary)
              .label("<")
          })
          .create_button(|b| {
            b.custom_id(&next_button_id)
              .style(ButtonStyle::Secondary)
              .label(">")
          })
        })
      })
    })
    .await?;

  // Loop through incoming interactions with the navigation buttons
  while let Some(press) = CollectComponentInteraction::new(ctx)
    // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
    // button was pressed
    .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
    .timeout(std::time::Duration::from_secs(300))
    .await
  {
    // Depending on which button was pressed, go to next or previous page
    if press.data.custom_id == next_button_id {
      page += 1;
      if page >= pages {
        page = 0;
      }
    } else if press.data.custom_id == prev_button_id {
      page = page.checked_sub(1).unwrap_or(pages - 1);
    } else {
      // This is an unrelated button interaction
      continue;
    }

    let pageee = render_page.clone()(page).await.unwrap();
    // Update the message with the new page contents
    press
      .create_interaction_response(ctx, |b| {
        b.kind(InteractionResponseType::UpdateMessage)
          .interaction_response_data(|b| b.content(pageee))
      })
      .await?;
  }
  msg
    .edit(ctx, |m| {
      m.components(|b| {
        b.create_action_row(|b| {
          b.create_button(|b| {
            b.custom_id(&prev_button_id)
              .style(ButtonStyle::Secondary)
              .label("<")
              .disabled(true)
          })
          .create_button(|b| {
            b.custom_id(&next_button_id)
              .style(ButtonStyle::Secondary)
              .label(">")
              .disabled(true)
          })
        })
      })
    })
    .await?;
  Ok(())
}

#[derive(FromRow)]
#[allow(dead_code)]
struct LinkRow {
  id: i64,
}
