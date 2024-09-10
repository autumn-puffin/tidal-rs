use serde::{Deserialize, Serialize};

use super::MixList;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
  pub id: u64,
  pub name: String,
  pub artist_types: Vec<ArtistType>,
  pub picture: Option<String>,
  pub url: String,
  pub mixes: MixList,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArtistType {
  Artist,
  Contributor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRelationship {
  pub id: u64,
  pub name: String,
  pub r#type: String,
  pub picture: Option<String>,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: std::collections::HashMap<String, serde_json::Value>,
}
