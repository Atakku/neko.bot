// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

//TODO: REWRITE
use std::path::Path;

use askama::{filters::format, Template};
use nbf::{Res, R};
use resvg::{
  tiny_skia::{Pixmap, Transform},
  usvg::{fontdb, Options, Tree, TreeParsing, TreeTextToPath},
};

use crate::cmd::top::apps::TopAppsRow;

pub fn render_svg<T>(template: T) -> Res<Vec<u8>>
where T: Template {
  let mut fontdb = fontdb::Database::new();
  fontdb.load_system_fonts();
  fontdb.load_font_file("font.ttf")?;

  let mut tree = Tree::from_str(&template.render()?, &Options::default()).unwrap();
  tree.convert_text(&fontdb);
  let mut pixmap = Pixmap::new(
    tree.size.width().round() as u32,
    tree.size.height().round() as u32,
  )
  .unwrap();
  let retree = resvg::Tree::from_usvg(&tree);
  retree.render(Transform::default(), &mut pixmap.as_mut());
  Ok(pixmap.encode_png()?)
}

pub async fn test_path<'a>(appid: u32, asset: &'a str) -> R {
  let name = format!(".cache/steam/{}/{}.jpg", asset, appid);
  let path = Path::new(&name);
  std::fs::create_dir_all(format!(".cache/steam/{}", asset))?;
  if !path.exists() {
    let req = reqwest::get(format!(
      "https://cdn.cloudflare.steamstatic.com/steam/apps/{}/{}.jpg",
      appid, asset
    ))
    .await?;
    if req.status() == 200 {
      std::fs::write(path, Into::<Vec<u8>>::into(req.bytes().await?))?;
    }
  }
  Ok(())
}

#[derive(Template, Clone)]
#[template(path = "top.svg", escape = "html")]
pub struct TopUI {
  //pub title: &'a String,
  //pub appid: i32,
  pub data: Vec<TopAppsRow>,
  //pub total: i32,
  pub page: u64,
  pub pages: u64,
}
