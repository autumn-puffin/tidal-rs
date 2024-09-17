use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::{AudioMode, AudioQuality};

use super::{ArtistRelationship, MediaMetadata};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
  pub id: u64,
  pub title: String,
  pub cover: Uuid,
  pub vibrant_color: String,
  pub url: String,
  pub video_cover: Option<String>,
  pub artists: Vec<ArtistRelationship>,
  pub audio_quality: AudioQuality,
  pub number_of_tracks: u64,
  pub number_of_videos: u64,
  pub duration: u64,
  pub stream_ready: bool,
  pub allow_streaming: bool,
  pub explicit: bool,
  pub media_metadata: MediaMetadata,
  pub audio_modes: Vec<AudioMode>,
  pub release_date: NaiveDate,
  pub stream_start_date: DateTime<Utc>,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumRelationship {
  pub id: u64,
  pub title: String,
  pub cover: Uuid,
  pub vibrant_color: String,
  pub url: String,
  pub video_cover: Option<Uuid>,
  pub release_date: NaiveDate,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: std::collections::HashMap<String, serde_json::Value>,
}