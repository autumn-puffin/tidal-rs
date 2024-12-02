use crate::api::{MediaType, PlaylistStyle};
use serde::{Deserialize, Serialize};

pub mod header_modules;
use header_modules::*;
pub mod collection_modules;
use collection_modules::*;
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum ModuleType {
  AlbumHeader(AlbumHeaderModule),                     //AlbumHeaderModule,
  AlbumItems(AlbumItemsCollectionModule),             //AlbumItemsCollectionModule,
  AlbumList(AlbumCollectionModule),                   //AlbumCollectionModule,
  ArtistHeader(ArtistHeaderModule),                   //ArtistHeaderModule
  ArticleList(ArticleCollectionModule),               //ArticleCollectionModule
  ArtistList(ArtistCollectionModule),                 //ArtistCollectionModule
  FeaturedPromotions,                                 //FeaturedPromotionsModule
  GenreHeader(GenreHeaderModule),                     //GenreHeaderModule
  HighlightModule(HighlightModule),                   //HighlightCollectionModule
  MixHeader(MixHeaderModule),                         //MixHeaderModule
  MixList(MixCollectionModule),                       //MixCollectionModule
  MixedTypesList(AnyMediaCollectionModule),           //AnyMediaCollectionModule
  MultipleTopPromotions(MultipleTopPromotionsModule), //MultipleTopPromotionsModule
  PageLinks(PageLinksCollectionModule),               //PageLinksCollectionModule
  PageLinksCloud(PageLinksCloudCollectionModule),     //PageLinksCloudCollectionModule
  PageLinksImage,                                     //PageLinksImagesCollectionModule
  PlaylistList(PlaylistCollectionModule),             //PlaylistCollectionModule
  Radio,                                              //RadioModule
  SingleTopPromotion,                                 //SingleTopPromotionModule
  Store,                                              //StoreModule
  TaskList,                                           //SetupTasksModule
  TextBlock(TextModule),                              //TextModule
  Ticketmaster,                                       //TicketMasterModule
  TrackList(TrackCollectionModule),                   //TrackCollectionModule
  Social(SocialModule),                               //SocialModule
  VideoList(VideoCollectionModule),                   //VideoCollectionModule
  ContributorHeader(ContributorHeaderModule),         //ContributorHeaderModule
  ItemListWithRoles,                                  //ContributionItemModule
  LiveSessionList,                                    //DJSessionModule
  #[default]
  Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighlightModule {
  pub playlist_style: Option<PlaylistStyle>,
  pub highlights: Vec<Highlight>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Highlight {
  pub title: String,
  pub item: MediaType,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextModule {
  pub icon: Option<String>,
  pub text: String,
  pub collapse: bool,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}
