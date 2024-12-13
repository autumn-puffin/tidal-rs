use spider::{Crawl as _, Target};
use std::{collections::HashSet, path::Path};
use tidal_rs::{
  api::Page,
  client::{auth::AuthCreds, Client, RefreshFlow as _},
};

mod error;
use error::Error;
mod spider;

fn main() {
  let mut client = load_authfile().unwrap();
  client.refresh().unwrap();

  let spider_path = Path::new("./spider/");
  std::fs::create_dir_all(spider_path).unwrap();

  let targets = get_initial_targets(&client, spider_path).unwrap();
  for target in &targets {
    println!("{}", target);
  }
  targets.iter().for_each(|target| {
    let path = match target {
      Target::Page(page) => page.clone(),
      Target::Album(album) => format!("album?albumId={}", album),
      Target::Artist(artist) => format!("artist?artistId={}", artist),
      Target::Mix(mix) => format!("mix?mixId={}", mix),
      // Target::Playlist(playlist) => format!("playlist?playlistId={}", playlist),
      Target::Profile(profile) => format!("profile?userId={}", profile),
      // Target::Track(track) => format!("track?trackId={}", track),
      Target::Video(video) => format!("video?videoId={}", video),
      _ => return,
    };
    let targets = get_page_targets(&client, spider_path, &path).unwrap();
    for target in &targets {
      println!("{}", target);
    }
  });
}

fn load_authfile() -> Result<Client, Error> {
  let creds = std::fs::read_to_string("./auth.json")?;

  let creds: AuthCreds = serde_json::from_str(&creds)?;
  let country = *creds.country_code().unwrap();
  let mut client = Client::new(creds.client_credentials().clone());
  client.set_auth_credentials(creds);
  client.set_country(country);
  Ok(client)
}

fn get_initial_targets(client: &Client, dump_path: &Path) -> Result<HashSet<Target>, Error> {
  let mut targets = HashSet::new();

  let home_page = "home";
  targets.extend(get_page_targets(client, dump_path, home_page)?);

  let explore_page = "explore";
  targets.extend(get_page_targets(client, dump_path, explore_page)?);

  Ok(targets)
}

fn get_page_targets(client: &Client, dump_path: &Path, page_path: &str) -> Result<HashSet<Target>, Error> {
  println!("getting page: {}", page_path);
  let page_json = client.get_page_response(page_path)?.text()?;
  if let Some(p) = dump_path.join(page_path).parent() {
    std::fs::create_dir_all(p)?;
  }
  std::fs::write(dump_path.join(format!("{}.json", page_path)), &page_json).unwrap();

  let page: Page = serde_json::from_str(&page_json)?;
  std::fs::write(dump_path.join(format!("{}.ron", page_path)), format!("{:#?}", page)).unwrap();

  let targets = page.identify_targets()?;
  Ok(targets)
}
