// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

//TODO: REWRITE
use nb_discord::schema::{Guilds, Members, Users};
use nbf::Res;
use sea_query::{
  Alias, Expr, Func, Iden, Order, PostgresQueryBuilder, Query, SelectStatement, WindowStatement,
  Write,
};
use sea_query_binder::SqlxBinder;
use sqlx::{postgres::PgRow, FromRow, PgPool};

use crate::schema::{Accounts, Apps, PlayData};

struct RowNumber;
impl Iden for RowNumber {
  fn unquoted(&self, s: &mut dyn Write) {
    write!(s, "ROW_NUMBER").unwrap();
  }
}

pub enum TopQueryArgs {
  TopApps {
    guild_id: i64,
    global: bool,
    user_id: Option<i64>,
  },
  TopGuilds {
    app_id: Option<i32>,
  },
  TopUsers {
    guild_id: i64,
    global: bool,
    app_id: Option<i32>,
    appcount: bool,
  },
}

pub fn top_query_builder(args: TopQueryArgs) -> SelectStatement {
  let mut qb = Query::select();
  qb.from(PlayData::Table);
  match args {
    TopQueryArgs::TopApps {
      guild_id,
      global,
      user_id,
    } => {
      qb.expr_as(
        Func::sum(Expr::col((PlayData::Table, PlayData::mins))),
        Alias::new("mins_sum"),
      );
      qb.expr_window_as(
        Func::cust(RowNumber),
        WindowStatement::new()
          .order_by_expr(
            Expr::sum(Expr::col((PlayData::Table, PlayData::mins))),
            Order::Desc,
          )
          .to_owned(),
        Alias::new("row_num"),
      );
      qb.columns([(Apps::Table, Apps::id), (Apps::Table, Apps::name)]);
      qb.from(Apps::Table);
      qb.and_where(Expr::col((Apps::Table, Apps::id)).equals((PlayData::Table, PlayData::app_id)));
      if let Some(user_id) = user_id {
        qb.from(Accounts::Table);
        qb.and_where(
          Expr::col((Accounts::Table, Accounts::id)).equals((PlayData::Table, PlayData::acc_id)),
        );
        qb.and_where(Expr::col((Accounts::Table, Accounts::discord_id)).eq(user_id));
      } else if !global {
        qb.from(Accounts::Table);
        qb.and_where(
          Expr::col((Accounts::Table, Accounts::id)).equals((PlayData::Table, PlayData::acc_id)),
        );
        qb.from(Members::Table);
        qb.and_where(
          Expr::col((Accounts::Table, Accounts::discord_id))
            .equals((Members::Table, Members::user_id)),
        );
        qb.and_where(Expr::col((Members::Table, Members::guild_id)).eq(guild_id));
      }
      qb.group_by_col((Apps::Table, Apps::id));
    }
    TopQueryArgs::TopGuilds { app_id } => {
      qb.expr_as(
        Func::sum(Expr::col((PlayData::Table, PlayData::mins))),
        Alias::new("mins_sum"),
      );
      qb.expr_window_as(
        Func::cust(RowNumber),
        WindowStatement::new()
          .order_by_expr(
            Expr::sum(Expr::col((PlayData::Table, PlayData::mins))),
            Order::Desc,
          )
          .to_owned(),
        Alias::new("row_num"),
      );
      qb.columns([(Guilds::Table, Guilds::id), (Guilds::Table, Guilds::name)]);
      qb.and_where(
        Expr::col((Accounts::Table, Accounts::id)).equals((PlayData::Table, PlayData::acc_id)),
      );
      qb.and_where(
        Expr::col((Accounts::Table, Accounts::discord_id))
          .equals((Members::Table, Members::user_id)),
      );
      qb.and_where(
        Expr::col((Guilds::Table, Guilds::id)).equals((Members::Table, Members::guild_id)),
      );
      qb.from(Accounts::Table);
      qb.from(Guilds::Table);
      qb.from(Members::Table);
      if let Some(app_id) = app_id {
        qb.and_where(Expr::col((PlayData::Table, PlayData::app_id)).eq(app_id));
      }
      qb.group_by_col((Guilds::Table, Guilds::id));
    }
    TopQueryArgs::TopUsers {
      guild_id,
      global,
      app_id,
      appcount,
    } => {
      if appcount {
        qb.expr_as(
          Func::count(Expr::col((PlayData::Table, PlayData::app_id))),
          Alias::new("mins_sum"),
        );
        qb.expr_window_as(
          Func::cust(RowNumber),
          WindowStatement::new()
            .order_by_expr(
              Expr::count(Expr::col((PlayData::Table, PlayData::app_id))),
              Order::Desc,
            )
            .to_owned(),
          Alias::new("row_num"),
        );
      } else {
        qb.expr_as(
          Func::sum(Expr::col((PlayData::Table, PlayData::mins))),
          Alias::new("mins_sum"),
        );
        qb.expr_window_as(
          Func::cust(RowNumber),
          WindowStatement::new()
            .order_by_expr(
              Expr::sum(Expr::col((PlayData::Table, PlayData::mins))),
              Order::Desc,
            )
            .to_owned(),
          Alias::new("row_num"),
        );
      }
      qb.columns([(Users::Table, Users::id), (Users::Table, Users::username)]);
      qb.from(Users::Table);
      qb.from(Accounts::Table);
      qb.and_where(
        Expr::col((Accounts::Table, Accounts::id)).equals((PlayData::Table, PlayData::acc_id)),
      );
      if let Some(app_id) = app_id {
        qb.and_where(Expr::col((PlayData::Table, PlayData::app_id)).eq(app_id));
      }
      if !global {
        qb.from(Members::Table);
        qb.and_where(
          Expr::col((Users::Table, Users::id)).equals((Members::Table, Members::user_id)),
        );
        qb.and_where(
          Expr::col((Accounts::Table, Accounts::discord_id))
            .equals((Members::Table, Members::user_id)),
        );
        qb.and_where(Expr::col((Members::Table, Members::guild_id)).eq(guild_id));
      } else {
        qb.and_where(
          Expr::col((Accounts::Table, Accounts::discord_id)).equals((Users::Table, Users::id)),
        );
      }
      qb.group_by_col((Users::Table, Users::id));
    }
  }
  qb.order_by(Alias::new("mins_sum"), Order::Desc);
  qb
}

pub async fn paged_query<T>(s: SelectStatement, p: &PgPool, size: u64, page: u64) -> Res<Vec<T>>
where
  T: std::marker::Send,
  T: std::marker::Unpin,
  T: for<'r> FromRow<'r, PgRow>,
{
  let mut sb = s.clone();
  if page == 0 {
    sb.limit(size + 1);
  } else {
    sb.limit(size + 2);
    sb.offset(page * size - 1);
  }
  let (q, v) = sb.build_sqlx(PostgresQueryBuilder);
  Ok(sqlx::query_as_with::<_, T, _>(&q, v).fetch_all(p).await?)
}
