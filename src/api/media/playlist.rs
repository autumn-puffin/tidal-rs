use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Artist;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
  pub uuid: Uuid,
  #[serde(skip)]
  pub etag: Option<String>,
  pub title: String,
  pub description: Option<String>,
  pub url: Option<String>,
  pub number_of_tracks: Option<u64>,
  pub number_of_videos: Option<u64>,
  pub duration: Option<u64>,
  pub promoted_artists: Option<Vec<Artist>>,
  pub image: Option<Uuid>,
  pub square_image: Option<Uuid>,
  pub last_item_added_at: Option<DateTime<Utc>>,
  pub r#type: PlaylistType,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaylistType {
  Artist,
  Editorial,
  Podcast,
  Private,
  Public,
  User,
}
