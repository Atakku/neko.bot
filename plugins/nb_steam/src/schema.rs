// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use sea_query::Iden;

#[derive(Iden)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum steam_accounts {
  Table,
  id,
  discord_id,
}
pub use steam_accounts as SteamAccounts;
pub use steam_accounts as Accounts;

#[derive(Iden)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum steam_apps {
  Table,
  id,
  name,
}
pub use steam_apps as SteamApps;
pub use steam_apps as Apps;

#[derive(Iden)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum steam_playdata {
  Table,
  id,
  app_id,
  acc_id,
  mins,
}
pub use steam_playdata as SteamPlayData;
pub use steam_playdata as PlayData;

#[derive(Iden)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum steam_playhist {
  Table,
  play_id,
  utc_day,
  mins,
}
pub use steam_playhist as SteamPlayHist;
pub use steam_playhist as PlayHist;
