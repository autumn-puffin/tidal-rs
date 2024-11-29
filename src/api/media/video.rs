use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Album, Artist};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
  pub id: u64,
  pub title: String,
  pub url: Option<String>,
  pub track_number: u64,
  pub volume_number: u64,
  pub duration: u64,
  pub popularity: u64,
  pub double_popularity: Option<f64>,
  pub stream_ready: bool,
  pub ad_supported_stream_ready: bool,
  pub allow_streaming: bool,
  pub explicit: bool,
  pub dj_ready: bool,
  pub stem_ready: bool,
  pub album: Option<Album>,
  pub artists: Vec<Artist>,
  pub stream_start_date: DateTime<Utc>,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}
