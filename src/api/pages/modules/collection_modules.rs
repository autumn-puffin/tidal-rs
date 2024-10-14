use serde::{Deserialize, Serialize};

use crate::api::{Album, Article, Artist, Layout, ListFormat, MediaType, Mix, PagingList, Playlist, PlaylistStyle, Track, Video};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionModule<T> {
  pub supports_paging: bool,
  pub paged_list: Option<PagingList<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaItemCollectionModule<T> {
  // pub block_filter: BlockFilter,
  pub list_format: ListFormat,
  pub mix_id: Option<String>,
  #[serde(flatten)]
  pub collection: CollectionModule<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumCollectionModule {
  pub list_format: Option<ListFormat>,
  #[serde(flatten)]
  pub collection: CollectionModule<Album>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleCollectionModule {
  #[serde(flatten)]
  pub collection: CollectionModule<Article>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistCollectionModule {
  #[serde(flatten)]
  pub collection: CollectionModule<Artist>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixCollectionModule {
  #[serde(flatten)]
  pub collection: CollectionModule<Mix>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistCollectionModule {
  pub playlist_style: Option<PlaylistStyle>,
  #[serde(flatten)]
  pub collection: CollectionModule<Playlist>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackCollectionModule {
  #[serde(flatten)]
  pub collection: CollectionModule<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoCollectionModule {
  pub layout: Option<Layout>,
  #[serde(flatten)]
  pub collection: CollectionModule<Video>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyMediaCollectionModule {
  #[serde(flatten)]
  pub collection: CollectionModule<MediaType>,
}
