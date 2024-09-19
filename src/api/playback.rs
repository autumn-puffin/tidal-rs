use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::{AssetPresentation, AudioMode, AudioQuality, PlaybackMode};

pub struct PlaybackInfoOptions {
  pub target_audio_quality: AudioQuality,
  pub playback_mode: PlaybackMode,
  pub asset_presentation: AssetPresentation,
  pub prefetch: bool,
}
impl PlaybackInfoOptions {
  pub fn new(target_audio_quality: AudioQuality, playback_mode: PlaybackMode, asset_presentation: AssetPresentation, prefetch: bool) -> Self {
    Self {
      target_audio_quality,
      playback_mode,
      asset_presentation,
      prefetch,
    }
  }
  pub fn get_query_params(&self) -> [(&str, &str); 3] {
    [
      ("audioquality", self.target_audio_quality.as_str()),
      ("playbackmode", self.playback_mode.as_str()),
      ("assetpresentation", self.asset_presentation.as_str()),
    ]
  }
}
impl Default for PlaybackInfoOptions {
  fn default() -> Self {
    Self {
      target_audio_quality: AudioQuality::High,
      playback_mode: PlaybackMode::Stream,
      asset_presentation: AssetPresentation::Full,
      prefetch: false,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackInfo {
  pub track_id: u64,
  pub asset_presentation: AssetPresentation,
  pub audio_mode: AudioMode,
  pub audio_quality: AudioQuality,
  pub streaming_session_id: Uuid,
  pub manifest_mime_type: String,
  pub manifest_hash: String,
  #[serde(with = "b64_mpd_serde")]
  pub manifest: dash_mpd::MPD,
  pub album_replay_gain: f64,
  pub album_peak_amplitude: f64,
  pub track_replay_gain: f64,
  pub track_peak_amplitude: f64,
  pub bit_depth: u64,
  pub sample_rate: u64,
}
impl PlaybackInfo {
  pub fn media_urls(&self) -> Vec<Url> {
    // TODO: replace with try block maybe when they become stable
    let opt = |p: &PlaybackInfo| -> Option<Vec<String>> {
      let t = p
        .manifest
        .periods
        .first()?
        .adaptations
        .first()?
        .representations
        .first()?
        .SegmentTemplate
        .clone()?;

      let mut urls = Vec::new();
      let init_url = t.initialization?;
      urls.push(init_url);
      let media_url_template = t.media?;
      let segs = t.SegmentTimeline?.segments;

      let mut total_segs = 0;
      for s in segs {
        let seg_count = s.r.unwrap_or(0) + 1;
        urls.reserve(seg_count as usize);
        for i in 1..=seg_count {
          let url = media_url_template.replace("$Number$", &(i + total_segs).to_string());
          urls.push(url);
        }
        total_segs += seg_count;
      }

      Some(urls)
    }(self);
    opt.unwrap_or_default().into_iter().map(|s| Url::parse(&s).unwrap()).collect()
  }
}

mod b64_mpd_serde {
  use base64::prelude::*;
  use serde::{de, Deserialize, Deserializer, Serializer};

  pub fn serialize<S>(mpd: &dash_mpd::MPD, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let str = mpd.to_string();
    let b64 = BASE64_STANDARD.encode(str.as_bytes());
    serializer.serialize_str(&b64)
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<dash_mpd::MPD, D::Error>
  where
    D: Deserializer<'de>,
  {
    let b64 = String::deserialize(deserializer)?;
    let bytes = BASE64_STANDARD
      .decode(b64)
      .map_err(|e| de::Error::custom(format!("Failed to decode Base64: {}", e)))?;
    let str = String::from_utf8(bytes).map_err(|e| de::Error::custom(format!("Failed to decode Base64: {}", e)))?;
    dash_mpd::parse(&str).map_err(de::Error::custom)
  }
}
