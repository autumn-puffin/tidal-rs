use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::{Album, Artist, MixList};

#[serde_flat_path::flat_path]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistHeaderModule {
  pub api_path: Option<String>,
  pub artist: Artist,
  #[flat_path(path = ["artistMix", "id"])]
  pub artist_mix: Option<String>,
  pub mixes: MixList,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumHeaderModule {
  pub album: Album,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreHeaderModule {
  pub mixes: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixHeaderModule {
  pub mix: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributorHeaderModule {
  pub artist: Artist,
}
