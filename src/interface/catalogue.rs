use super::auth::Auth;
use crate::{api::Page, Result};
use isocountry::CountryCode;

pub mod album_catalogue;
pub mod artist_catalogue;
pub mod search;
pub mod track_catalogue;
pub mod video_catalogue;

pub trait Catalogue: Auth {
  fn get_country(&self) -> Result<&CountryCode>;
  fn get_page(&self, page: &str) -> Result<Page>;

  fn get_home_page(&self) -> Result<Page> {
    self.get_page("home")
  }
  fn get_explore_page(&self) -> Result<Page> {
    self.get_page("explore")
  }
  fn get_mix_page(&self, mix_id: &str) -> Result<Page> {
    let path = format!("mix?mixId={}", mix_id);
    self.get_page(path.as_str())
  }
  fn get_artist_page(&self, artist_id: &u64) -> Result<Page> {
    let path = format!("artist?artistId={}", artist_id);
    self.get_page(path.as_str())
  }
  fn get_album_page(&self, album_id: &u64) -> Result<Page> {
    let path = format!("album?albumId={}", album_id);
    self.get_page(path.as_str())
  }
}
