use serde::{Deserialize, Serialize};

pub mod album;
pub mod article;
pub mod artist;
pub mod image;
pub mod lyrics;
pub mod mix;
pub mod playlist;
pub mod profile;
pub mod track;
pub mod video;
pub use album::*;
pub use article::*;
pub use artist::*;
pub use image::*;
pub use lyrics::*;
pub use mix::*;
pub use playlist::*;
pub use profile::*;
pub use track::*;
pub use video::*;

use super::MediaTag;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type", content = "item")]
pub enum MediaType {
  Album(Album),
  Artist(Artist),
  Playlist(Playlist),
  Profile(serde_json::Value),
  Mix(Mix),
  Track(Track),
  Video(Video),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "item")]
pub enum MediaItem {
  Track(Track),
  Video(Video),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaItemRecommendation {
  Track(Track),
  Video(Video),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
  pub tags: Vec<MediaTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaRecommendation {
  #[serde(flatten)]
  media: MediaItemRecommendation,
  sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaCredit {
  pub r#type: String,
  pub contributors: Vec<Artist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialItem {
  pub r#type: String,
  pub url: String,
}