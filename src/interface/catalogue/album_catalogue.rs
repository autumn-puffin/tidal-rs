use crate::{api, Result};

use super::Catalogue;

pub trait AlbumCatalogue: Catalogue {
  fn get_album(&self, album_id: &u64) -> Result<api::Album>;
  fn get_album_credits(&self, album_id: &u64, include_contributors: bool) -> Result<Vec<api::MediaCredit>>;
  fn get_album_items(&self, album_id: &u64, offset: &u64, limit: &u64) -> Result<api::Paging<api::MediaItem>>;
  fn get_album_items_with_credits(
    &self,
    album_id: &u64,
    offset: &u64,
    limit: &u64,
    include_contributors: bool,
  ) -> Result<api::Paging<api::MediaItem>>;
}
