use serde::{Deserialize, Serialize};

pub mod album;
pub mod article;
pub mod artist;
pub mod image;
pub mod mix;
pub mod playlist;
pub mod profile;
pub mod track;
pub mod video;
pub use album::*;
pub use article::*;
pub use artist::*;
pub use image::*;
pub use mix::*;
pub use playlist::*;
pub use profile::*;
pub use track::*;
pub use video::*;

use super::MediaTag;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
  pub tags: Vec<MediaTag>,
}
