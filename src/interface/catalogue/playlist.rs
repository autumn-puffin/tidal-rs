use reqwest::blocking::Response;
use uuid::Uuid;

use crate::{api, Result};

use super::Catalogue;

pub trait PlaylistCatalogue: Catalogue {
  fn get_playlist(&self, playlist_id: &Uuid) -> Result<api::Playlist>;
  fn get_playlist_items(&self, playlist_id: &Uuid, offset: &u64, limit: &u64) -> Result<api::Paging<api::MediaItem>>;
  fn get_playlist_recommendations(&self, playlist_id: &Uuid, offset: &u64, limit: &u64) -> Result<api::Paging<api::MediaItem>>;
}

pub trait PlaylistCollection {
  fn add_favorite_playlist(&self, parent_folder_id: Option<&Uuid>, playlist_id: &Uuid) -> Result<Response>;
  fn create_playlist(&self, parent_folder_id: Option<&Uuid>, name: &str, description: &str, is_public: bool) -> Result<Response>;
  fn remove_playlist(&self, playlist_id: &Uuid) -> Result<Response>;
  fn move_playlist(&self, parent_folder_id: Option<&Uuid>, playlist_id: &Uuid) -> Result<Response>;
  fn edit_playlist(&self, playlist_id: &Uuid, name: &str, description: &str) -> Result<Response>;
  fn publish_playlist(&self, playlist_id: &Uuid) -> Result<Response>;
  fn unpublish_playlist(&self, playlist_id: &Uuid) -> Result<Response>;
  fn get_collection_playlist(&self, playlist_id: &Uuid) -> Result<Response>;

  // TODO: figure out what trns does here
  fn create_folder(&self, parent_folder_id: Option<&Uuid>, name: &str, _trns: Option<&str>) -> Result<Response>;
  fn remove_folder(&self, folder_id: &Uuid) -> Result<Response>;
  fn get_folder_items(&self, folder_id: Option<&Uuid>) -> Result<Response>;
  fn move_folder(&self, parent_folder_id: Option<&Uuid>, folder_id: &Uuid) -> Result<Response>;
  fn rename_folder(&self, folder_id: &Uuid, name: &str) -> Result<Response>;
}
