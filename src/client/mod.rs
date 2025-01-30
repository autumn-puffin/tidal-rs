//! TIDAL-rs' main client implementation for the Tidal API, implimenting all of the available interfaces.
//!
//! The `Client` struct is a basic all-inclusive blocking client for the Tidal API, there are also
//! standalone clients for individual parts of the API, such as the `AuthClient`, and the `CatalogueClient`

use std::collections::HashMap;

pub use crate::interface::{
  auth::{flows::*, *},
  catalogue::{track::*, *},
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
use isocountry::CountryCode;
use reqwest::{
  blocking::{Client as ReqwestClient, Response},
  header::HeaderMap,
};
use serde::{Deserialize, Serialize};
use tidal_rs_macros::{base_url, basic_auth, bearer_auth, body, body_form_url_encoded, client, delete, get, headers, post, query, shared_query};
use url::Url;
use uuid::Uuid;

/// Standalone auth client implimentation
pub mod auth;
use auth::{AuthClient, AuthCreds, TokenResponse};

/// Standalone catalogue client implimentation
pub mod catalogue;
use catalogue::CatalogueClient;

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
  pub fn get_client_id(&self) -> &str {
    self.client_credentials.id()
  }
  pub fn get_client_secret(&self) -> &str {
    self.client_credentials.secret()
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

  fn bearer_auth(&self) -> Option<&str> {
    self.get_credentials().ok().map(|a| a.access_token())
  }

  fn shared_query(&self) -> [(&str, &str); 3] {
    let cc = self.get_country().map_or("WW", |cc| cc.alpha2());
    [("countryCode", cc), ("locale", "en_US"), ("deviceType", "BROWSER")]
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
#[client(self.http_client)]
#[base_url("https://api.tidal.com/v1")]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[shared_query(&self.shared_query())]
impl Sessions for Client {
  #[get("/sessions")]
  fn get_session_from_auth(&self) -> Result<Session> {}

  #[get(format!("/sessions/{session_id}"))]
  fn get_session(&self, session_id: &str) -> Result<Session> {}
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
#[client(self.http_client)]
#[base_url("https://auth.tidal.com/v1/")]
#[basic_auth((self.get_client_id(), Some(self.get_client_secret())))]
#[shared_query(&self.shared_query())]
impl DeviceFlow for Client {
  #[post("oauth2/device_authorization")]
  #[query(&[
    ("scope", "r_usr+w_usr+w_sub"), 
    ("client_id", self.get_client_id())
  ])]
  fn device_login_init(&self) -> Result<crate::interface::auth::flows::DeviceFlowResponse> {}

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
#[client(self.http_client)]
#[base_url("https://api.tidal.com/v1")]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[shared_query(&self.shared_query())]
impl Users for Client {
  #[get(format!("/users/{user_id}"))]
  fn get_user(&self, user_id: &u64) -> Result<User> {}

  #[get(format!("/users/{user_id}/subscription"))]
  fn get_user_subscription(&self, user_id: &u64) -> Result<UserSubscription> {}

  #[get(format!("/users/{user_id}/clients"))]
  fn get_user_clients(&self, user_id: &u64) -> Result<Paging<UserClient>> {}

  #[post(format!("/users/{client_id}/clients"))]
  #[body(&[("clientName", name), ("clientId", &client_id.to_string())])]
  #[body_form_url_encoded]
  fn authorize_client(&self, client_id: &u64, name: &str) -> Result<()> {}

  #[delete(format!("/users/{client_id}/clients"))]
  fn deauthorize_client(&self, client_id: &u64) -> Result<()> {}
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
#[client(self.http_client)]
#[base_url("https://api.tidal.com/v1")]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[shared_query(&self.shared_query())]
impl TrackCatalogue for Client {
  #[get(format!("/tracks/{track_id}"))]
  fn get_track(&self, track_id: &u64) -> Result<Track> {}

  #[get(format!("/tracks/{track_id}/credits"))]
  #[query(&[
    ("limit", &limit.to_string()), 
    ("includeContributors", &include_contributors.to_string())
  ])]
  fn get_track_credits(&self, track_id: &u64, limit: &u64, include_contributors: bool) -> Result<Vec<MediaCredit>> {}

  #[get(format!("/tracks/{track_id}/mix"))]
  fn get_track_mix_id(&self, track_id: &u64) -> Result<MixId> {}

  #[get(format!("/tracks/{track_id}/lyrics"))]
  fn get_track_lyrics(&self, track_id: &u64) -> Result<Lyrics> {}

  #[get(format!("/tracks/{track_id}/recommendations"))]
  #[query(&[("limit", &limit.to_string())])]
  fn get_track_recommendations(&self, track_id: &u64, limit: &u64) -> Result<Paging<MediaRecommendation>> {}

  #[get(format!("/tracks/{track_id}/playbackinfo"))]
  #[query(&options.get_query_params())]
  #[headers([
    ("x-tidal-token", self.client_credentials.id()),
    ("x-tidal-prefetch", &options.prefetch.to_string()),
    ("x-tidal-streamingsessionid", &self.streaming_session_id.to_string()),
  ])]
  fn playback_info_for_track(&self, track_id: &u64, options: &PlaybackInfoOptions) -> Result<PlaybackInfo> {}
}
#[client(self.http_client)]
#[base_url("https://api.tidal.com/v1")]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[shared_query(&self.shared_query())]
impl VideoCatalogue for Client {
  #[get(format!("/videos/{video_id}"))]
  fn get_video(&self, video_id: &u64) -> Result<crate::api::Video> {}

  #[get(format!("/videos/{video_id}/recommendations"))]
  #[query(&[("limit", &limit.to_string())])]
  fn get_video_recommendations(&self, video_id: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaRecommendation>> {}

  #[get(format!("/videos/{video_id}/playbackinfo"))]
  #[query(&options.get_query_params())]
  fn playback_info_for_video(&self, video_id: &u64, options: &crate::api::PlaybackInfoOptions) -> Result<crate::api::PlaybackInfo> {}
}
#[client(self.http_client)]
#[base_url("https://api.tidal.com/v1")]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[shared_query(&self.shared_query())]
impl ArtistCatalogue for Client {
  #[get(format!("/artists/{artist_id}"))]
  fn get_artist(&self, artist_id: &u64) -> Result<crate::api::Artist> {}

  #[get(format!("/artists/{artist_id}/bio"))]
  fn get_artist_bio(&self, artist_id: &u64) -> Result<crate::api::ArtistBio> {}

  #[get(format!("/artists/{artist_id}/mix"))]
  fn get_artist_mix_id(&self, artist_id: &u64) -> Result<crate::api::MixId> {}

  #[get(format!("/artists/{artist_id}/toptracks"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  fn get_artist_top_tracks(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::Track>> {}

  #[get(format!("/artists/{artist_id}/videos"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  fn get_artist_videos(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::Video>> {}

  #[get(format!("/artists/{artist_id}/albums"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  fn get_artist_albums(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::Album>> {}
}
#[client(self.http_client)]
#[base_url("https://api.tidal.com/v1")]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[shared_query(&self.shared_query())]
impl AlbumCatalogue for Client {
  #[get(format!("/albums/{album_id}"))]
  fn get_album(&self, album_id: &u64) -> Result<crate::api::Album> {}

  #[get(format!("/albums/{album_id}/credits"))]
  #[query(&[("includeContributors", &include_contributors.to_string())])]
  fn get_album_credits(&self, album_id: &u64, include_contributors: bool) -> Result<Vec<crate::api::MediaCredit>> {}

  #[get(format!("/albums/{album_id}/items"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  fn get_album_items(&self, album_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaItem>> {}

  #[get(format!("/albums/{album_id}/items/credits"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
    ("includeContributors", &include_contributors.to_string())
  ])]
  fn get_album_items_with_credits(
    &self,
    album_id: &u64,
    offset: &u64,
    limit: &u64,
    include_contributors: bool,
  ) -> Result<crate::api::Paging<crate::api::MediaItem>> {
  }
}
#[client(self.http_client)]
#[base_url("https://api.tidal.com/v1")]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[shared_query(&self.shared_query())]
impl PlaylistCatalogue for Client {
  fn get_playlist(&self, playlist_id: &Uuid) -> Result<crate::api::Playlist> {
    let endpoint = Endpoint::Playlists(playlist_id);
    let response = self.get_endpoint_response(endpoint)?;
    let etag = response.headers().get("ETag").map(|v| v.to_str().unwrap().to_string());
    let mut playlist: crate::api::Playlist = response.json()?;
    playlist.etag = etag;
    Ok(playlist)
  }

  #[get(format!("/playlists/{playlist_id}/items"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  fn get_playlist_items(&self, playlist_id: &Uuid, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaItem>> {}

  #[get(format!("/playlists/{playlist_id}/recommendations/items"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  fn get_playlist_recommendations(&self, playlist_id: &Uuid, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaItem>> {}
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
