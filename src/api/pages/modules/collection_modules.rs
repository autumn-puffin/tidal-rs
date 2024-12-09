use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::api::{
  Album, Article, Artist, Layout, ListFormat, MediaItem, MediaType, Mix, PageItem, PagingList, Playlist, PlaylistStyle, PromotionElement, SocialItem, Track, Video
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionModule<T> {
  pub supports_paging: bool,
  pub paged_list: Option<PagingList<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageLinksCollectionModule {
  pub paged_list: PagingList<PageItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageLinksCloudCollectionModule {
  pub paged_list: PagingList<PageItem>,
  pub lines: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleTopPromotionsModule {
  pub items: Vec<PromotionElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaItemCollectionModule<T> {
  // pub block_filter: BlockFilter,
  pub list_format: ListFormat,
  pub mix_id: Option<String>,
  #[serde(flatten)]
  pub collection: CollectionModule<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumCollectionModule {
  pub list_format: Option<ListFormat>,
  #[serde(flatten)]
  pub collection: CollectionModule<Album>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleCollectionModule {
  #[serde(flatten)]
  pub collection: CollectionModule<Article>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistCollectionModule {
  #[serde(flatten)]
  pub collection: CollectionModule<Artist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixCollectionModule {
  #[serde(flatten)]
  pub collection: CollectionModule<Mix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCollectionModule {
  pub playlist_style: Option<PlaylistStyle>,
  #[serde(flatten)]
  pub collection: CollectionModule<Playlist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackCollectionModule {
  #[serde(flatten)]
  pub collection: CollectionModule<Track>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoCollectionModule {
  pub layout: Option<Layout>,
  #[serde(flatten)]
  pub collection: CollectionModule<Video>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyMediaCollectionModule {
  #[serde(flatten)]
  pub collection: CollectionModule<MediaType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumItemsCollectionModule {
  pub play_button: bool,
  pub shuffle_button: bool,
  pub release_date: NaiveDate,
  pub list_format: ListFormat,
  pub copyright: String,
  #[serde(flatten)]
  pub collection: CollectionModule<MediaItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialModule {
  pub social_profiles: Vec<SocialItem>,
  pub social_links: Vec<SocialItem>,
}