//! TIDAL-rs' main client implementation for the Tidal API, implimenting all of the available interfaces.
//!
//! The `Client` struct is a basic all-inclusive blocking client for the Tidal API, there are also
//! standalone clients for individual parts of the API, such as the `AuthClient`, and the `CatalogueClient`

use std::collections::HashMap;

pub use crate::interface::{
  auth::{flows::*, *},
  catalogue::{track_catalogue::*, *},
  users::*,
};
use crate::{
  api::{
    Lyrics, MediaCredit, MediaRecommendation, MixId, Page, Paging, PlaybackInfo, PlaybackInfoOptions, Session, Track, User, UserClient,
    UserSubscription,
  },
  endpoints::Endpoint,
  utils, Result,
};
use album_catalogue::AlbumCatalogue;
use artist_catalogue::ArtistCatalogue;
use isocountry::CountryCode;
use playlist_catalogue::PlaylistCatalogue;
use reqwest::{
  blocking::{Client as ReqwestClient, Response},
  header::HeaderMap,
};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

/// Standalone auth client implimentation
pub mod auth;
use auth::{AuthClient, AuthCreds, TokenResponse};

/// Standalone catalogue client implimentation
pub mod catalogue;
use catalogue::CatalogueClient;
use video_catalogue::VideoCatalogue;

/// A client for interacting with the Tidal API, implimenting all of the available interfaces.
pub struct Client {
  http_client: ReqwestClient,
  client_credentials: ClientCreds,
  auth_credentials: Option<AuthCreds>,
  scopes: Vec<String>,
  redirect_uri: Option<String>,
  country: Option<CountryCode>,
  streaming_session_id: Uuid,
}
impl Client {
  /// Create a new `Client` with the given `ClientCreds`
  pub fn new(client_credentials: ClientCreds) -> Self {
    let credentials = client_credentials;
    let streaming_session_id = Uuid::new_v4();

    Self {
      http_client: ReqwestClient::new(),
      client_credentials: credentials,
      auth_credentials: None,
      scopes: Vec::new(),
      redirect_uri: None,
      country: None,
      streaming_session_id,
    }
  }
  /// Create an `AuthClient` from the `Client`
  pub fn as_auth(&self) -> AuthClient {
    AuthClient::new(self.client_credentials.clone())
  }
  /// Create a `CatalogueClient` from the `Client`
  pub fn as_catalogue(&self) -> CatalogueClient {
    CatalogueClient::new(self.client_credentials.clone())
  }
  /// Set the redirect uri for code flow auth
  pub fn set_redirect_uri(&mut self, uri: String) {
    self.redirect_uri = Some(uri);
  }
  /// set the country code for the client
  pub fn set_country(&mut self, country: CountryCode) {
    self.country = Some(country);
  }
  /// Set the auth credentials for the client
  pub fn set_auth_credentials(&mut self, auth_credentials: AuthCreds) {
    self.auth_credentials = Some(auth_credentials)
  }
  /// get the client credentials of the `Client`
  pub fn get_client_credentials(&self) -> &ClientCreds {
    &self.client_credentials
  }
  /// get the auth credentials of the `Client`
  pub fn get_auth_credentials(&self) -> Option<&AuthCreds> {
    self.auth_credentials.as_ref()
  }
  /// get the scopes of the `Client`
  pub fn get_scopes(&self) -> &[String] {
    &self.scopes
  }
  /// get the country code of the `Client`
  pub fn get_country_code(&self) -> Option<&CountryCode> {
    self.country.as_ref()
  }
  /// get the streaming session id of the `Client`
  pub fn get_streaming_session_id(&self) -> &Uuid {
    &self.streaming_session_id
  }
  /// return true if the client is authrnticated and has a refresh token
  pub fn can_refresh(&self) -> bool {
    self.auth_credentials.as_ref().map_or(false, |creds| creds.refresh_token().is_some())
  }
  /// run a closure that may need to refresh the auth token
  ///
  /// ```rust
  /// let closure = |c: &mut Client| -> Result<Page> { c.get_page("home") }
  ///
  /// let page = client.map_refresh(closure)?;
  /// // if an unauthenticated error is returned from map_refresh, the session is unable to be refreshed
  ///
  /// ```
  pub fn map_refresh<F, T>(&mut self, mut func: F) -> Result<T>
  where
    F: FnMut(&mut Self) -> Result<T>,
  {
    match func(self) {
      Ok(val) => Ok(val),
      Err(e) => {
        if e.is_unauthenticated() && self.can_refresh() {
          self.refresh()?;
          func(self)
        } else {
          Err(e)
        }
      }
    }
  }
  /// get a page as a `Response`
  pub fn get_page_response(&self, page: &str) -> Result<Response> {
    let endpoint = Endpoint::Pages(page);
    let query = &[("countryCode", self.get_country()?.alpha2()), ("deviceType", "BROWSER")];

    self.get_helper(endpoint, Some(query), None, None)
  }
  /// get an endpoint as a `Response`
  pub fn get_endpoint_response(&self, endpoint: Endpoint) -> Result<Response> {
    let query = &[("countryCode", self.get_country()?.alpha2()), ("locale", "en_US")];
    self.get_helper(endpoint, Some(query), None, None)
  }
}
impl Client {
  /// Helper function for making oauth requests
  fn oauth_helper(&mut self, endpoint: Endpoint, grant: GrantType, params: Option<&[(&str, &str)]>) -> Result<()> {
    let client_creds = &self.client_credentials;
    let res = utils::oauth_request_helper(&self.http_client, endpoint, grant, client_creds, params).send()?;
    if !res.status().is_success() {
      return Err(utils::res_to_error(res));
    }
    let auth_creds = AuthCreds::new(grant, self.client_credentials.clone(), res.json::<TokenResponse>()?);
    let country = auth_creds.auth_user().map(|user| user.country_code);

    self.auth_credentials = Some(auth_creds);
    self.country = country;
    Ok(())
  }
  /// Helper function for making get requests
  fn get_helper(
    &self,
    endpoint: Endpoint,
    query: Option<&[(&str, &str)]>,
    form: Option<&[(&str, &str)]>,
    headers: Option<&[(&str, &str)]>,
  ) -> Result<Response> {
    let auth = self.get_credentials().ok();
    let headers = &HashMap::from_iter(headers.unwrap_or_default().iter().map(|(k, v)| (k.to_string(), v.to_string())));
    let headers = HeaderMap::try_from(headers).unwrap();
    let res = self
      .http_client
      .get(endpoint.to_string())
      .query(query.unwrap_or_default())
      .form(form.unwrap_or_default())
      .headers(headers)
      .bearer_auth(auth.map(|a| a.access_token()).unwrap_or_default())
      .send()?;
    if !res.status().is_success() {
      return Err(utils::res_to_error(res));
    }
    Ok(res)
  }
  /// Helper function for making post requests
  fn post_helper(
    &self,
    endpoint: Endpoint,
    query: Option<&[(&str, &str)]>,
    form: Option<&[(&str, &str)]>,
    headers: Option<&[(&str, &str)]>,
  ) -> Result<Response> {
    let auth = self.get_credentials().ok();
    let headers = &HashMap::from_iter(headers.unwrap_or_default().iter().map(|(k, v)| (k.to_string(), v.to_string())));
    let headers = HeaderMap::try_from(headers).unwrap();
    let res = self
      .http_client
      .post(endpoint.to_string())
      .query(query.unwrap_or_default())
      .form(form.unwrap_or_default())
      .headers(headers)
      .bearer_auth(auth.map(|a| a.access_token()).unwrap_or_default())
      .send()?;
    if !res.status().is_success() {
      return Err(utils::res_to_error(res));
    }
    Ok(res)
  }
  /// Helper function for making delete requests
  fn delete_helper(
    &self,
    endpoint: Endpoint,
    query: Option<&[(&str, &str)]>,
    form: Option<&[(&str, &str)]>,
    headers: Option<&[(&str, &str)]>,
  ) -> Result<Response> {
    let auth = self.get_credentials().ok();
    let headers = &HashMap::from_iter(headers.unwrap_or_default().iter().map(|(k, v)| (k.to_string(), v.to_string())));
    let headers = HeaderMap::try_from(headers).unwrap();
    let res = self
      .http_client
      .delete(endpoint.to_string())
      .query(query.unwrap_or_default())
      .form(form.unwrap_or_default())
      .headers(headers)
      .bearer_auth(auth.map(|a| a.access_token()).unwrap_or_default())
      .send()?;
    if !res.status().is_success() {
      return Err(utils::res_to_error(res));
    }
    Ok(res)
  }
}
impl Auth for Client {
  type Credentials = AuthCreds;
  fn get_credentials(&self) -> Result<&AuthCreds> {
    self.auth_credentials.as_ref().ok_or(AuthError::Unauthenticated.into())
  }
  fn get_credentials_mut(&mut self) -> Result<&mut AuthCreds> {
    self.auth_credentials.as_mut().ok_or(AuthError::Unauthenticated.into())
  }
}
impl Sessions for Client {
  fn get_session_from_auth(&self) -> Result<Session> {
    let endpoint = Endpoint::SessionsOfBearer;
    let res = self.get_helper(endpoint, None, None, None)?;
    Ok(res.json()?)
  }

  fn get_session(&self, session_id: &str) -> Result<Session> {
    let endpoint = Endpoint::Sessions(session_id);
    let res = self.get_helper(endpoint, None, None, None)?;
    Ok(res.json()?)
  }
}
impl ClientFlow for Client {
  fn client_login(&mut self) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::ClientCredentials;

    self.oauth_helper(endpoint, grant, None)
  }
}
impl UserFlow for Client {
  fn user_login_init(&self) -> Result<UserFlowInfo> {
    let redirect_uri = self.redirect_uri.as_ref().ok_or(AuthError::Unauthenticated)?;
    let scopes = self.scopes.join(" ");
    let (pkce_challenge, pkce_verifier) = utils::new_pkce_pair();

    let auth_url = Url::parse_with_params(
      &Endpoint::LoginAuthorize.to_string(),
      &[
        ("response_type", "code"),
        ("client_id", self.client_credentials.id()),
        ("redirect_uri", redirect_uri),
        ("scope", &scopes),
        ("code_challenge_method", "S256"),
        ("code_challenge", &pkce_challenge),
      ],
    )?
    .to_string();

    Ok(UserFlowInfo::new(auth_url, pkce_verifier))
  }

  fn user_login_finalize(&mut self, code: String, info: UserFlowInfo) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::AuthorizationCode;
    let verifier = info.verifier();
    let id = self.client_credentials.id().to_owned();
    let redirect_uri = self.redirect_uri.as_ref().ok_or(AuthError::MissingRedirectUri)?.clone();

    let params = &[
      ("code_verifier", verifier),
      ("code", &code),
      ("client_id", &id),
      ("redirect_uri", &redirect_uri),
    ];

    self.oauth_helper(endpoint, grant, Some(params))
  }
}
impl DeviceFlow for Client {
  fn device_login_init(&self) -> Result<crate::interface::auth::flows::DeviceFlowResponse> {
    let endpoint = Endpoint::OAuth2DeviceAuth;
    let params = &[("scope", "r_usr+w_usr+w_sub"), ("client_id", self.client_credentials.id())];

    let res = self.post_helper(endpoint, None, Some(params), None)?;
    Ok(res.json()?)
  }

  fn try_device_login_finalize(&mut self, response: &crate::interface::auth::flows::DeviceFlowResponse) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::DeviceCode;
    let id = self.client_credentials.id().to_owned();
    let params = &[("scope", "r_usr+w_usr+w_sub"), ("client_id", &id), ("device_code", &response.device_code)];

    self.oauth_helper(endpoint, grant, Some(params))
  }
}
impl RefreshFlow for Client {
  fn refresh(&mut self) -> Result<()> {
    self
      .auth_credentials
      .as_mut()
      .ok_or(AuthError::Unauthenticated)?
      .refresh_with_http_client(&self.http_client)
  }
}
impl Users for Client {
  fn get_user(&self, user_id: &u64) -> Result<User> {
    let endpoint = Endpoint::Users(user_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn get_user_subscription(&self, user_id: &u64) -> Result<UserSubscription> {
    let endpoint = Endpoint::UsersSubscription(user_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn get_user_clients(&self, user_id: &u64) -> Result<Paging<UserClient>> {
    let endpoint = Endpoint::UsersClients(user_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn authorize_client(&self, client_id: &u64, name: &str) -> Result<()> {
    let endpoint = Endpoint::UsersClients(client_id);
    let form = &[("clientName", name), ("clientId", &client_id.to_string())];
    self.post_helper(endpoint, None, Some(form), None)?;
    Ok(())
  }

  fn deauthorize_client(&self, client_id: &u64) -> Result<()> {
    let endpoint = Endpoint::UsersClients(client_id);
    self.delete_helper(endpoint, None, None, None)?;
    Ok(())
  }
}
impl Catalogue for Client {
  fn get_country(&self) -> Result<&CountryCode> {
    self.country.as_ref().ok_or(AuthError::Unauthenticated.into())
  }

  fn get_page(&self, page: &str) -> Result<Page> {
    let res = self.get_page_response(page)?;
    Ok(res.json()?)
  }
}
impl TrackCatalogue for Client {
  fn get_track(&self, track_id: &u64) -> Result<Track> {
    let endpoint = Endpoint::Tracks(track_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn get_track_credits(&self, track_id: &u64, limit: &u64, include_contributors: bool) -> Result<Vec<MediaCredit>> {
    let endpoint = Endpoint::TracksCredits(track_id);
    let query = &[
      ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
      ("limit", &limit.to_string()),
      ("includeContributors", &include_contributors.to_string()),
    ];
    let res = self.get_helper(endpoint, Some(query), None, None)?;
    Ok(res.json()?)
  }

  fn get_track_mix_id(&self, track_id: &u64) -> Result<MixId> {
    let endpoint = Endpoint::TracksMix(track_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn get_track_lyrics(&self, track_id: &u64) -> Result<Lyrics> {
    let endpoint = Endpoint::TracksLyrics(track_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn get_track_recommendations(&self, track_id: &u64, limit: &u64) -> Result<Paging<MediaRecommendation>> {
    let endpoint = Endpoint::TracksRecommendations(track_id);
    let query: &[(&str, &str)] = &[
      ("limit", &limit.to_string()),
      ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
    ];
    let res = self.get_helper(endpoint, Some(query), None, None)?;
    Ok(res.json()?)
  }

  fn playback_info_for_track(&self, track_id: &u64, options: &PlaybackInfoOptions) -> Result<PlaybackInfo> {
    let endpoint = Endpoint::TracksPlaybackinfo(track_id);
    let query = &options.get_query_params();
    let headers = &[
      ("x-tidal-token", self.client_credentials.id()),
      ("x-tidal-prefetch", &options.prefetch.to_string()),
      ("x-tidal-streamingsessionid", &self.streaming_session_id.to_string()),
    ];
    let res = self.get_helper(endpoint, Some(query), None, Some(headers))?;
    Ok(res.json()?)
  }
}
impl VideoCatalogue for Client {
  fn get_video(&self, video_id: &u64) -> Result<crate::api::Video> {
    let endpoint = Endpoint::Videos(video_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn get_video_recommendations(&self, video_id: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaRecommendation>> {
    let endpoint = Endpoint::VideosRecommendations(video_id);
    let query: &[(&str, &str)] = &[
      ("limit", &limit.to_string()),
      ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
    ];
    let res = self.get_helper(endpoint, Some(query), None, None)?;
    Ok(res.json()?)
  }

  fn playback_info_for_video(&self, video_id: &u64, options: &crate::api::PlaybackInfoOptions) -> Result<crate::api::PlaybackInfo> {
    let endpoint = Endpoint::VideosPlaybackinfo(video_id);
    let query = &options.get_query_params();
    let headers = &[
      ("x-tidal-token", self.client_credentials.id()),
      ("x-tidal-prefetch", &options.prefetch.to_string()),
      ("x-tidal-streamingsessionid", &self.streaming_session_id.to_string()),
    ];
    let res = self.get_helper(endpoint, Some(query), None, Some(headers))?;
    Ok(res.json()?)
  }
}
impl ArtistCatalogue for Client {
  fn get_artist(&self, artist_id: &u64) -> Result<crate::api::Artist> {
    let endpoint = Endpoint::Artists(artist_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn get_artist_bio(&self, artist_id: &u64) -> Result<crate::api::ArtistBio> {
    let endpoint = Endpoint::ArtistsBio(artist_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn get_artist_mix_id(&self, artist_id: &u64) -> Result<crate::api::MixId> {
    let endpoint = Endpoint::ArtistsMix(artist_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn get_artist_top_tracks(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::Track>> {
    let endpoint = Endpoint::ArtistsTopTracks(artist_id);
    let query: &[(&str, &str)] = &[
      ("offset", &offset.to_string()),
      ("limit", &limit.to_string()),
      ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
    ];
    let res = self.get_helper(endpoint, Some(query), None, None)?;
    Ok(res.json()?)
  }

  fn get_artist_videos(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::Video>> {
    let endpoint = Endpoint::ArtistsVideos(artist_id);
    let query: &[(&str, &str)] = &[
      ("offset", &offset.to_string()),
      ("limit", &limit.to_string()),
      ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
    ];
    let res = self.get_helper(endpoint, Some(query), None, None)?;
    Ok(res.json()?)
  }

  fn get_artist_albums(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::Album>> {
    let endpoint = Endpoint::ArtistsAlbums(artist_id);
    let query: &[(&str, &str)] = &[
      ("offset", &offset.to_string()),
      ("limit", &limit.to_string()),
      ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
    ];
    let res = self.get_helper(endpoint, Some(query), None, None)?;
    Ok(res.json()?)
  }
}
impl AlbumCatalogue for Client {
  fn get_album(&self, album_id: &u64) -> Result<crate::api::Album> {
    let endpoint = Endpoint::Albums(album_id);
    Ok(self.get_endpoint_response(endpoint)?.json()?)
  }

  fn get_album_credits(&self, album_id: &u64, include_contributors: bool) -> Result<Vec<crate::api::MediaCredit>> {
    let endpoint = Endpoint::AlbumsCredits(album_id);
    let query: &[(&str, &str)] = &[
      ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
      ("includeContributors", &include_contributors.to_string()),
    ];
    Ok(self.get_helper(endpoint, Some(query), None, None)?.json()?)
  }

  fn get_album_items(&self, album_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaItem>> {
    let endpint = Endpoint::AlbumsItems(album_id);
    let query: &[(&str, &str)] = &[
      ("offset", &offset.to_string()),
      ("limit", &limit.to_string()),
      ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
    ];
    Ok(self.get_helper(endpint, Some(query), None, None)?.json()?)
  }

  fn get_album_items_with_credits(
    &self,
    album_id: &u64,
    offset: &u64,
    limit: &u64,
    include_contributors: bool,
  ) -> Result<crate::api::Paging<crate::api::MediaItem>> {
    let endpint = Endpoint::AlbumsItems(album_id);
    let query: &[(&str, &str)] = &[
      ("offset", &offset.to_string()),
      ("limit", &limit.to_string()),
      ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
      ("includeContributors", &include_contributors.to_string()),
    ];
    Ok(self.get_helper(endpint, Some(query), None, None)?.json()?)
  }
}
impl PlaylistCatalogue for Client {
    fn get_playlist(&self, playlist_id: &Uuid) -> Result<crate::api::Playlist> {
        let endpoint = Endpoint::Playlists(playlist_id);
        let response = self.get_endpoint_response(endpoint)?;
        let etag = response.headers().get("ETag").map(|v| v.to_str().unwrap().to_string());
        let mut playlist: crate::api::Playlist = response.json()?;
        playlist.etag = etag;
        Ok(playlist)
    }

    fn get_playlist_items(&self, playlist_id: &Uuid, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaItem>> {
        let endpoint = Endpoint::PlaylistsItems(playlist_id);
        let query: &[(&str, &str)] = &[
            ("offset", &offset.to_string()),
            ("limit", &limit.to_string()),
            ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
            ("locale", "en_US"),
        ];
        Ok(self.get_helper(endpoint, Some(query), None, None)?.json()?)
    }

    fn get_playlist_recommendations(&self, playlist_id: &Uuid, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaItem>> {
        let endpoint = Endpoint::PlaylistsRecommendations(playlist_id);
        let query: &[(&str, &str)] = &[
            ("offset", &offset.to_string()),
            ("limit", &limit.to_string()),
            ("countryCode", self.country.unwrap_or(CountryCode::USA).alpha2()),
        ];
        Ok(self.get_helper(endpoint, Some(query), None, None)?.json()?)
    }
}

/// A simple struct for storing client credentials, with a custom Debug impl that redacts the client secret.
#[derive(Clone, Serialize, Deserialize)]
pub struct ClientCreds {
  client_id: String,
  client_secret: String,
}
impl ClientCreds {
  /// Create a new `ClientCreds` struct with the given client_id and client_secret
  pub fn new(client_id: String, client_secret: String) -> Self {
    Self { client_id, client_secret }
  }
  /// Returns the client_id
  pub fn id(&self) -> &str {
    &self.client_id
  }
  /// Returns the client_secret
  pub fn secret(&self) -> &str {
    &self.client_secret
  }
  /// Returns the client creds as a tuple of (id, secret)
  pub fn as_tuple(&self) -> (&str, &str) {
    (&self.client_id, &self.client_secret)
  }
}
impl std::fmt::Debug for ClientCreds {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ClientCreds")
      .field("client_id", &self.client_id)
      .field("client_secret", &"[REDACTED]")
      .finish()
  }
}
