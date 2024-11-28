use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::{AudioMode, AudioQuality};

use super::{Album, Artist, MediaMetadata, MixList};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
  pub id: u64,
  pub title: String,
  pub url: Option<String>,
  pub track_number: u64,
  pub volume_number: u64,
  pub duration: u64,
  pub popularity: u64,
  pub double_popularity: Option<f64>,
  pub replay_gain: f64,
  pub peak: Option<f64>,
  pub stream_ready: bool,
  pub ad_supported_stream_ready: bool,
  pub allow_streaming: bool,
  #[serde(default)]
  pub premium_streaming_only: bool,
  pub explicit: bool,
  pub dj_ready: bool,
  pub stem_ready: bool,
  pub editable: bool,
  pub isrc: Option<String>,
  pub copyright: Option<String>,
  pub audio_quality: AudioQuality,
  pub mixes: MixList,
  pub album: Album,
  pub artist: Option<Artist>,
  pub artists: Vec<Artist>,
  pub media_metadata: MediaMetadata,
  pub audio_modes: Vec<AudioMode>,
  pub stream_start_date: DateTime<Utc>,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}
