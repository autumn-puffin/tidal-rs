use serde::{Deserialize, Serialize};

use super::ArtistRelationship;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
  pub id: u64,
  pub title: String,
  pub cover: String,
  pub vibrant_color: String,
  pub video_cover: Option<String>,
  pub url: String,
  pub artists: Vec<ArtistRelationship>,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: std::collections::HashMap<String, serde_json::Value>,
}
