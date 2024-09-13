use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AudioQuality {
  Low,
  High,
  Lossless,
  HiRes,
  HiResLossless,
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
