use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{AssetPresentation, AudioMode, AudioQuality, PlaybackMode};

pub struct PlaybackInfoOptions {
  pub target_audio_quality: AudioQuality,
  pub playback_mode: PlaybackMode,
  pub asset_presentation: AssetPresentation,
  pub prefetch: bool,
}
impl PlaybackInfoOptions {
  pub fn new(target_audio_quality: AudioQuality, playback_mode: PlaybackMode, asset_presentation: AssetPresentation, prefetch: bool) -> Self {
    Self {
      target_audio_quality,
      playback_mode,
      asset_presentation,
      prefetch,
    }
  }
  pub fn get_query_params(&self) -> [(&str, &str); 3] {
    [
      ("audioquality", self.target_audio_quality.as_str()),
      ("playbackmode", self.playback_mode.as_str()),
      ("assetpresentation", self.asset_presentation.as_str()),
    ]
  }
}
impl Default for PlaybackInfoOptions {
  fn default() -> Self {
    Self {
      target_audio_quality: AudioQuality::High,
      playback_mode: PlaybackMode::Stream,
      asset_presentation: AssetPresentation::Full,
      prefetch: false,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackInfo {
  pub track_id: u64,
  pub asset_presentation: AssetPresentation,
  pub audio_mode: AudioMode,
  pub audio_quality: AudioQuality,
  pub streaming_session_id: Uuid,
  pub manifest_mime_type: String,
  pub manifest_hash: String,
  pub manifest: String,
  pub album_replay_gain: f64,
  pub album_peak_amplitude: f64,
  pub track_replay_gain: f64,
  pub track_peak_amplitude: f64,
  pub bit_depth: u64,
  pub sample_rate: u64,
}
