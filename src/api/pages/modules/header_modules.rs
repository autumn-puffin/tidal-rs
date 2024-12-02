use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::{Album, Artist, Mix, MixList};

#[serde_flat_path::flat_path]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistHeaderModule {
  pub api_path: Option<String>,
  pub artist: Artist,
  // this causes errors if null
  // #[flat_path(path = ["artistMix", "id"])]
  // pub artist_mix: Option<String>,
  pub mixes: MixList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumHeaderModule {
  pub album: Album,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreHeaderModule {
  pub mixes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixHeaderModule {
  pub mix: Mix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributorHeaderModule {
  pub artist: Artist,
}
