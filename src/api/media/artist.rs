use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::MixList;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
  pub id: u64,
  pub name: String,
  pub artist_types: Option<Vec<ArtistType>>,
  pub artist_roles: Option<Vec<ArtistRole>>,
  pub picture: Option<Uuid>,
  pub url: Option<String>,
  #[serde(default)]
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
pub struct ArtistRole {
  pub category: String,
  pub category_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRelationship {
  pub r#type: String,
  pub id: u64,
  pub name: String,
  pub picture: Option<Uuid>,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: std::collections::HashMap<String, serde_json::Value>,
}
