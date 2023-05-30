// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

//TODO: REWRITE
use nb_poise::Ctx;
use nbf::R;
use regex::Regex;
use sqlx::{FromRow, PgPool};

use crate::query::{paged_query, top_query_builder, TopQueryArgs};

// Top guilds
#[poise::command(prefix_command, slash_command, guild_only)]
pub(crate) async fn guilds(ctx: Ctx<'_>, app: Option<String>) -> R {
  // TODO: move into shared
  let regex: Regex = Regex::new(r"\d+")?;
  let qb = top_query_builder(TopQueryArgs::TopGuilds {
    app_id: app.and_then(|a| {
      regex
        .captures(a.as_str())?
        .get(0)
        .and_then(|m| m.as_str().parse().ok())
    }),
  })
  .to_owned();

  let pool = ctx.data().read().await.borrow::<PgPool>()?.clone();

  let data = paged_query::<TopGuildsRow>(qb, &pool, 8, 0).await?;

  ctx
    .send(|g| {
      g.content(format!(
        "The bot is currenly being rewritten. Fancy UI will come back shortly\n```{}```",
        data
          .into_iter()
          .map(|o| format!("#{} | {} | {}\n", o.row_num, o.mins_sum / 60, o.name))
          .collect::<String>()
      ))
    })
    .await?;
  Ok(())
}

#[derive(FromRow)]
#[allow(dead_code)]
struct TopGuildsRow {
  id: i64,
  name: String,
  mins_sum: i64,
  row_num: i64,
}
