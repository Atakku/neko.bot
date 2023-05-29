// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use reqwest::Error as ReqwestError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Clone)]
pub struct SteamAPI {
  api_base: String,
  api_key: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct ApiResponse<T> {
  #[serde(alias = "friendslist")]
  pub response: T,
}

#[derive(Deserialize)]
pub struct FriendList {
  pub friends: Vec<Friend>,
  //relationship
  //friend_since
}

#[serde_as]
#[derive(Deserialize)]
pub struct Friend {
  #[serde_as(as = "DisplayFromStr")]
  pub steamid: i64,
}

#[derive(Deserialize, Serialize, Default)]
pub struct OwnedGames {
  pub game_count: i32,
  pub games: Vec<OwnedGame>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct OwnedGame {
  pub appid: i32,
  pub name: String,
  //img_icon_url: String,
  //playtime_2weeks: Option<i32>,
  pub playtime_forever: i32,
  //playtime_windows_forever: i32,
  //playtime_mac_forever: i32,
  //playtime_linux_forever: i32,
  //rtime_last_played: i64,
}

#[allow(dead_code)]
impl SteamAPI {
  pub fn new<S>(key: S) -> SteamAPI
  where S: Into<String> {
    SteamAPI {
      api_base: "https://api.steampowered.com".into(),
      api_key: key.into(),
    }
  }

  async fn api<'a, E, R, P>(&self, e: E, p: P) -> Result<R, ReqwestError>
  where
    E: Into<String>,
    R: DeserializeOwned,
    P: Into<String>,
  {
    Self::get(&format!(
      "{}/{}/v0001/?key={}&format=json{}",
      &self.api_base,
      e.into(),
      &self.api_key,
      p.into()
    ))
    .await
  }

  async fn get<'a, S, T>(url: S) -> Result<T, ReqwestError>
  where
    S: Into<String>,
    T: DeserializeOwned,
  {
    reqwest::get(url.into()).await?.json::<T>().await
  }

  pub async fn get_owned_games(&self, id: &i64) -> Result<ApiResponse<OwnedGames>, ReqwestError> {
    self
      .api(
        "IPlayerService/GetOwnedGames",
        &format!(
          "&steamid={}&include_appinfo=true&include_played_free_games=true",
          id
        ),
      )
      .await
  }

  pub async fn get_friend_list(&self, id: &i64) -> Result<ApiResponse<FriendList>, ReqwestError> {
    self
      .api(
        "ISteamUser/GetFriendList",
        format!("&steamid={}&relationship=friend", id),
      )
      .await
  }
}
