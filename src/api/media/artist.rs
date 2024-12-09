use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::MixList;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
  pub id: Option<u64>,
  pub name: String,
  pub r#type: Option<String>,
  pub artist_types: Option<Vec<ArtistType>>,
  pub artist_roles: Option<Vec<ArtistRole>>,
  pub picture: Option<Uuid>,
  pub url: Option<String>,
  #[serde(default)]
  pub mixes: MixList,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistBio {
  pub source: String,
  pub last_updated: DateTime<Utc>,
  pub text: String,
  pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArtistType {
  Artist,
  Contributor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRole {
  pub category: String,
  pub category_id: i64,
}
