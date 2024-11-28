use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::{AudioMode, AudioQuality};

use super::{Artist, MediaMetadata};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
  pub id: u64,
  pub title: String,
  pub cover: Uuid,
  pub vibrant_color: Option<String>,
  pub url: Option<String>,
  pub video_cover: Option<String>,
  pub artists: Vec<Artist>,
  pub audio_quality: AudioQuality,
  pub number_of_tracks: u64,
  pub number_of_videos: u64,
  pub duration: u64,
  pub stream_ready: bool,
  pub allow_streaming: bool,
  pub explicit: bool,
  pub media_metadata: MediaMetadata,
  pub audio_modes: Vec<AudioMode>,
  pub release_date: Option<NaiveDate>,
  pub stream_start_date: Option<DateTime<Utc>>,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}
