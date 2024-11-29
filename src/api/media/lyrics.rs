use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
  pub track_id: u64,
  pub lyrics_provider: String,
  pub provider_commontrack_id: String,
  pub provider_lyrics_id: String,
  pub lyrics: String,
  pub subtitles: String,
  pub is_right_to_left: bool,
}
