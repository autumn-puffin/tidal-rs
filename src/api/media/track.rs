use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::{AudioMode, AudioQuality};

use super::{AlbumRelationship, ArtistRelationship, MediaMetadata, MixList};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
  pub id: u64,
  pub title: String,
  pub url: String,
  pub track_number: u64,
  pub volume_number: u64,
  pub duration: u64,
  pub popularity: u64,
  pub double_popularity: f64,
  pub replay_gain: f64,
  pub stream_ready: bool,
  pub ad_supported_stream_ready: bool,
  pub allow_streaming: bool,
  pub explicit: bool,
  pub dj_ready: bool,
  pub stem_ready: bool,
  pub editable: bool,
  pub audio_quality: AudioQuality,
  pub mixes: MixList,
  pub album: AlbumRelationship,
  pub artists: Vec<ArtistRelationship>,
  pub media_metadata: MediaMetadata,
  pub audio_modes: Vec<AudioMode>,
  pub stream_start_date: DateTime<Utc>,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: std::collections::HashMap<String, serde_json::Value>,
}
