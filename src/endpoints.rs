//! Endpoints for the Tidal API

use std::fmt::{Display, Formatter};

use uuid::Uuid;

/// Base urls for accessing the Tidal API, defaults to the production urls
pub mod base_urls {
  pub use prod::*;
  pub mod prod {
    pub const API_URL_V1: &str = "https://api.tidal.com/v1/";
    pub const API_URL_V2: &str = "https://api.tidal.com/v2/";
    pub const OPEN_API_URL: &str = "https://openapi.tidal.com/v2/";
    pub const AUTH_URL: &str = "https://auth.tidal.com/v1/";
    pub const LOGIN_URL: &str = "https://login.tidal.com/";
  }
  pub mod stage {
    pub const API_URL_V1: &str = "https://api.stage.tidal.com/v1/";
    pub const API_URL_V2: &str = "https://api.stage.tidal.com/v2/";
    pub const OPEN_API_URL: &str = "https://openapi.stage.tidal.com/v2/";
    pub const AUTH_URL: &str = "https://auth.stage.tidal.com/v1/";
    pub const LOGIN_URL: &str = "https://login.stage.tidal.com/";
  }
}

/// Enum of available endpoints for the Tidal API
#[derive(Debug, Clone, Copy)]
#[deprecated = "Deprecated in favor of the attribute macros"]
#[allow(dead_code)]
pub(crate) enum Endpoint<'a> {
  OAuth2Token,
  OAuth2DeviceAuth,
  LoginAuthorize,
  Users(&'a u64),
  UsersSubscription(&'a u64),
  UsersClients(&'a u64),
  Sessions(&'a str),
  SessionsOfBearer,
  Pages(&'a str),
  Tracks(&'a u64),
  TracksCredits(&'a u64),
  TracksMix(&'a u64),
  TracksLyrics(&'a u64),
  TracksRecommendations(&'a u64),
  TracksPlaybackinfo(&'a u64),
  Videos(&'a u64),
  VideosRecommendations(&'a u64),
  VideosPlaybackinfo(&'a u64),
  Artists(&'a u64),
  ArtistsBio(&'a u64),
  ArtistsLinks(&'a u64),
  ArtistsMix(&'a u64),
  ArtistsTopTracks(&'a u64),
  ArtistsVideos(&'a u64),
  ArtistsAlbums(&'a u64),
  Albums(&'a u64),
  AlbumsCredits(&'a u64),
  AlbumsItems(&'a u64),
  AlbumsItemsWithCredits(&'a u64),
  AlbumsReview(&'a u64),
  Playlists(&'a Uuid),
  PlaylistsItems(&'a Uuid),
  PlaylistsRecommendations(&'a Uuid),
}
#[allow(deprecated)]
impl Display for Endpoint<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    let (base, path) = match self {
      Self::OAuth2Token => (base_urls::AUTH_URL, "oauth2/token".to_owned()),
      Self::OAuth2DeviceAuth => (base_urls::AUTH_URL, "oauth2/device_authorization".to_owned()),
      Self::LoginAuthorize => (base_urls::LOGIN_URL, "authorize".to_owned()),
      Self::Sessions(id) => (base_urls::API_URL_V1, format!("sessions/{id}")),
      Self::SessionsOfBearer => (base_urls::API_URL_V1, "sessions".to_owned()),
      Self::Users(id) => (base_urls::API_URL_V1, format!("users/{id}")),
      Self::UsersSubscription(id) => (base_urls::API_URL_V1, format!("users/{id}/subscription")),
      Self::UsersClients(id) => (base_urls::API_URL_V1, format!("users/{id}/clients")),
      Self::Pages(path) => (base_urls::API_URL_V1, format!("pages/{path}")),
      Self::Tracks(id) => (base_urls::API_URL_V1, format!("tracks/{id}")),
      Self::TracksCredits(id) => (base_urls::API_URL_V1, format!("tracks/{id}/credits")),
      Self::TracksMix(id) => (base_urls::API_URL_V1, format!("tracks/{id}/mix")),
      Self::TracksLyrics(id) => (base_urls::API_URL_V1, format!("tracks/{id}/lyrics")),
      Self::TracksRecommendations(id) => (base_urls::API_URL_V1, format!("tracks/{id}/recommendations")),
      Self::TracksPlaybackinfo(id) => (base_urls::API_URL_V1, format!("tracks/{id}/playbackinfo")),
      Self::Videos(id) => (base_urls::API_URL_V1, format!("videos/{id}")),
      Self::VideosRecommendations(id) => (base_urls::API_URL_V1, format!("videos/{id}/recommendations")),
      Self::VideosPlaybackinfo(id) => (base_urls::API_URL_V1, format!("videos/{id}/playbackinfo")),
      Self::Artists(id) => (base_urls::API_URL_V1, format!("artists/{id}")),
      Self::ArtistsBio(id) => (base_urls::API_URL_V1, format!("artists/{id}/bio")),
      Self::ArtistsLinks(id) => (base_urls::API_URL_V1, format!("artists/{id}/links")),
      Self::ArtistsMix(id) => (base_urls::API_URL_V1, format!("artists/{id}/mix")),
      Self::ArtistsTopTracks(id) => (base_urls::API_URL_V1, format!("artists/{id}/toptracks")),
      Self::ArtistsVideos(id) => (base_urls::API_URL_V1, format!("artists/{id}/videos")),
      Self::ArtistsAlbums(id) => (base_urls::API_URL_V1, format!("artists/{id}/albums")),
      Self::Albums(id) => (base_urls::API_URL_V1, format!("albums/{id}")),
      Self::AlbumsCredits(id) => (base_urls::API_URL_V1, format!("albums/{id}/credits")),
      Self::AlbumsItems(id) => (base_urls::API_URL_V1, format!("albums/{id}/items")),
      Self::AlbumsItemsWithCredits(id) => (base_urls::API_URL_V1, format!("albums/{id}/items/credits")),
      Self::AlbumsReview(id) => (base_urls::API_URL_V1, format!("albums/{id}/review")),
      Self::Playlists(id) => (base_urls::API_URL_V1, format!("playlists/{id}")),
      Self::PlaylistsItems(id) => (base_urls::API_URL_V1, format!("playlists/{id}/items")),
      Self::PlaylistsRecommendations(id) => (base_urls::API_URL_V1, format!("playlists/{id}/recommendations/items")),
    };
    write! {f, "{}{}", base, path}
  }
}
