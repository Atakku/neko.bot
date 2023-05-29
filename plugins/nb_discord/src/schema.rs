// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use sea_query::Iden;

#[derive(Iden)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum discord_guilds {
  Table,
  id,
  name,
  icon,
}
pub use discord_guilds as DiscordGuilds;
pub use discord_guilds as Guilds;

#[derive(Iden)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum discord_members {
  Table,
  guild_id,
  user_id,
  nickname,
  avatar,
}
pub use discord_members as DiscordMembers;
pub use discord_members as Members;

#[derive(Iden)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum discord_users {
  Table,
  id,
  username,
  nickname,
  avatar,
}
pub use discord_users as DiscordUsers;
pub use discord_users as Users;

#[derive(Iden)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum discord_whitelist {
  Table,
  id,
}
pub use discord_whitelist as DiscordWhitelist;
pub use discord_whitelist as Whitelist;
