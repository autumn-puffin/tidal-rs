use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ArtistRelationship;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
  pub uuid: Uuid,
  pub title: String,
  pub description: String,
  pub url: String,
  pub number_of_tracks: u64,
  pub number_of_videos: u64,
  pub duration: u64,
  pub promoted_artists: Vec<ArtistRelationship>,
  pub image: Uuid,
  pub square_image: Uuid,
  pub last_item_added_at: DateTime<Utc>,
  pub r#type: PlaylistType,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaylistType {
  Artist,
  Editorial,
  Podcast,
  Private,
  Public,
  User,
}
