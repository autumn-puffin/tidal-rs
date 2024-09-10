use serde::{Deserialize, Serialize};

use crate::api::{Album, Artist, MediaType, Mix, PagingList, Playlist, Track, Video};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum ModuleType {
  AlbumHeader,                              //AlbumHeaderModule,
  AlbumItems,                               //AlbumItemsCollectionModule,
  AlbumList(CollectionModule<Album>),       //AlbumCollectionModule,
  ArtistHeader,                             //ArtistHeaderModule
  ArticleList,                              //ArticleCollectionModule
  ArtistList(CollectionModule<Artist>),     //ArtistCollectionModule
  FeaturedPromotions,                       //FeaturedPromotionsModule
  GenreHeader,                              //GenreHeaderModule
  HighlightModule(HighlightModule),         //HighlightCollectionModule
  MixHeader,                                //MixHeaderModule
  MixList(CollectionModule<Mix>),           //MixCollectionModule
  MixedTypesList,                           //AnyMediaCollectionModule
  MultipleTopPromotions,                    //MultipleTopPromotionsModule
  PageLinks,                                //PageLinksCollectionModule
  PageLinksCloud,                           //PageLinksCloudCollectionModule
  PageLinksImage,                           //PageLinksImagesCollectionModule
  PlaylistList(CollectionModule<Playlist>), //PlaylistCollectionModule
  Radio,                                    //RadioModule
  SingleTopPromotion,                       //SingleTopPromotionModule
  Store,                                    //StoreModule
  TaskList(CollectionModule<Track>),        //SetupTasksModule
  TextBlock,                                //TextModule
  Ticketmaster,                             //TicketMasterModule
  TrackList,                                //TrackCollectionModule
  Social,                                   //SocialModule
  VideoList(CollectionModule<Video>),       //VideoCollectionModule
  ContributorHeader,                        //ContributorHeaderModule
  ItemListWithRoles,                        //ContributionItemModule
  LiveSessionList,                          //DJSessionModule
  #[default]
  Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightModule {
  pub playlist_style: String,
  pub highlights: Vec<Highlight>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Highlight {
  pub title: String,
  pub item: MediaType,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionModule<T> {
  pub supports_paging: bool,
  pub paged_list: Option<PagingList<T>>,
}
