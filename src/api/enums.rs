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
pub enum ModuleType {
  AlbumHeader,
  AlbumItems,
  AlbumList,
  ArticleList,
  ArtistHeader,
  ArtistList,
  ContributorHeader,
  FeaturedPromotions,
  GenreHeader,
  HighlightModule,
  LiveSessionList,
  MixHeader,
  MixList,
  MixedTypesList,
  MultipleTopPromotions,
  PageLinks,
  PageLinksCloud,
  PageLinksImage,
  PlaylistList,
  Radio,
  SingleTopPromotion,
  Store,
  TaskList,
  TextBlock,
  Ticketmaster,
  TrackList,
  Social,
  VideoList,
  ItemListWithRoles,
}
