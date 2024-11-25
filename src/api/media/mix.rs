use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Image;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixId {
  pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MixType {
  ArtistMix,
  DailyMix,
  DiscoveryMix,
  GenreMix,
  HistoryAlltimeMix,
  HistoryMonthlyMix,
  HistoryYearlyMix,
  MasterArtistMix,
  MasterNewReleaseMix,
  MasterTrackMix,
  NewReleaseMix,
  ProducerMix,
  TrackMix,
  SongwriterMix,
  Unknown,
  VideoDailyMix,
  WelcomeMix,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MixList {
  pub artist_mix: Option<String>,
  pub master_artist_mix: Option<String>,
  pub track_mix: Option<String>,
  pub master_track_mix: Option<String>,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mix {
  pub id: String,
  pub title: Option<String>,
  pub sub_title: Option<String>,
  pub short_subtitle: Option<String>,
  pub mix_type: MixType,
  pub images: HashMap<String, Image>,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}
