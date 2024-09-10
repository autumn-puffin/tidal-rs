use serde::{Deserialize, Serialize};

pub mod album;
pub use album::*;
pub mod artist;
pub use artist::*;
pub mod playlist;
pub use playlist::*;
pub mod profile;
pub use profile::*;
pub mod mix;
pub use mix::*;
pub mod track;
pub use track::*;
pub mod video;
pub use video::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type", content = "item")]
pub enum MediaType {
  Album(Album),
  Artist(serde_json::Value),
  Playlist(serde_json::Value),
  Profile(serde_json::Value),
  Mix(serde_json::Value),
  Track(serde_json::Value),
  Video(serde_json::Value),
}
