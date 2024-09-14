use reqwest::blocking::Response;

use crate::{api, Result};

use super::{Catalogue, PlaybackInfoOptions};

pub trait TrackCatalogue: Catalogue {
  fn get_track(&self, track_id: &u64) -> Result<api::Track>;
  fn playback_info_for_track(&self, track_id: &u64, options: &PlaybackInfoOptions) -> Result<Response>;
}
