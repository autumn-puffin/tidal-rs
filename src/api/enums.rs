use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudioQuality {
  Low,
  High,
  Lossless,
  HiRes,
  HiResLossless,
}
impl AudioQuality {
  pub fn as_str(&self) -> &str {
    match self {
      AudioQuality::Low => "LOW",
      AudioQuality::High => "HIGH",
      AudioQuality::Lossless => "LOSSLESS",
      AudioQuality::HiRes => "HI_RES",
      AudioQuality::HiResLossless => "HI_RES_LOSSLESS",
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionType {
  Intro,
  Premium,
  PremiumPlus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentType {
  AdyenBoleto,
  AdyenCreditCard,
  AdyenIdeal,
  AdyenSofort,
  ApplePay,
  CashApp,
  Fortumo,
  GooglePlayBilling,
  Paypal,
  Venmo,
  Vivo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ListFormat {
  Covers,
  Numbers,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaylistStyle {
  Artist,
  ArtistUpdated,
  By,
  ByExtended,
  ByUpdated,
  Description,
  DescriptionUpdated,
  Default,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Layout {
  List,
  Grid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaTag {
  DolbyAtmos,
  HiresLossless,
  Lossless,
  Mqa,
  Sony360ra,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudioMode {
  Stereo,
  DolbyAtmos,
  Sony360ra,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaybackMode {
  Offline,
  Stream,
}
impl PlaybackMode {
  pub fn as_str(&self) -> &str {
    match self {
      PlaybackMode::Offline => "OFFLINE",
      PlaybackMode::Stream => "STREAM",
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetPresentation {
  Full,
  Preview,
}
impl AssetPresentation {
  pub fn as_str(&self) -> &str {
    match self {
      AssetPresentation::Full => "FULL",
      AssetPresentation::Preview => "PREVIEW",
    }
  }
}
