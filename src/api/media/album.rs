use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::{AudioMode, AudioQuality};

use super::{Artist, MediaMetadata};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
  pub id: u64,
  pub title: String,
  pub cover: Option<Uuid>,
  pub vibrant_color: Option<String>,
  pub url: Option<String>,
  pub video_cover: Option<String>,
  pub artists: Option<Vec<Artist>>,
  pub audio_quality: Option<AudioQuality>,
  pub number_of_tracks: Option<u64>,
  pub number_of_videos: Option<u64>,
  pub duration: Option<u64>,
  pub stream_ready: Option<bool>,
  pub allow_streaming: Option<bool>,
  pub explicit: Option<bool>,
  pub media_metadata: Option<MediaMetadata>,
  pub audio_modes: Option<Vec<AudioMode>>,
  pub release_date: Option<NaiveDate>,
  pub stream_start_date: Option<DateTime<Utc>>,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}
