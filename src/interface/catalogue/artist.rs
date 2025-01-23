use crate::{api, Result};

use super::Catalogue;

pub trait ArtistCatalogue: Catalogue {
  fn get_artist(&self, artist_id: &u64) -> Result<api::Artist>;
  fn get_artist_bio(&self, artist_id: &u64) -> Result<api::ArtistBio>;
  fn get_artist_mix_id(&self, artist_id: &u64) -> Result<api::MixId>;
  fn get_artist_top_tracks(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<api::Paging<api::Track>>;
  fn get_artist_videos(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<api::Paging<api::Video>>;
  fn get_artist_albums(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<api::Paging<api::Album>>;
}
