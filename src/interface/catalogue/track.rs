use crate::{api, Result};

use super::Catalogue;

pub trait TrackCatalogue: Catalogue {
  fn get_track(&self, track_id: &u64) -> Result<api::Track>;
  fn get_track_credits(&self, track_id: &u64, limit: &u64, include_contributors: bool) -> Result<Vec<api::MediaCredit>>;
  fn get_track_mix_id(&self, track_id: &u64) -> Result<api::MixId>;
  fn get_track_lyrics(&self, track_id: &u64) -> Result<api::Lyrics>;
  fn get_track_recommendations(&self, track_id: &u64, limit: &u64) -> Result<api::Paging<api::MediaRecommendation>>;
  fn playback_info_for_track(&self, track_id: &u64, options: &api::PlaybackInfoOptions) -> Result<api::PlaybackInfo>;
}
