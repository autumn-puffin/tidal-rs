use crate::{api, Result};

use super::Catalogue;

pub trait VideoCatalogue: Catalogue {
  fn get_video(&self, video_id: &u64) -> Result<api::Video>;
  fn get_video_recommendations(&self, video_id: &u64, offset: &u64, limit: &u64) -> Result<api::Paging<api::MediaRecommendation>>;
  fn playback_info_for_video(&self, video_id: &u64, options: &api::PlaybackInfoOptions) -> Result<api::PlaybackInfo>;
}
