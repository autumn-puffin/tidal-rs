use uuid::Uuid;

use crate::{api, Result};

use super::Catalogue;

pub trait PlaylistCatalogue: Catalogue {
  fn get_playlist(&self, playlist_id: &Uuid) -> Result<api::Playlist>;
  fn get_playlist_items(&self, playlist_id: &Uuid, offset: &u64, limit: &u64) -> Result<api::Paging<api::MediaItem>>;
  fn get_playlist_recommendations(&self, playlist_id: &Uuid, offset: &u64, limit: &u64) -> Result<api::Paging<api::MediaItem>>;
}