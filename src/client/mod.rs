//! TIDAL-rs' main client implementation for the Tidal API, implimenting all of the available interfaces.
//!
//! The `Client` struct is a basic all-inclusive blocking client for the Tidal API, there are also
//! standalone clients for individual parts of the API, such as the `AuthClient`, and the `CatalogueClient`

use std::collections::HashMap;

use crate::{
  api::{
    Lyrics, MediaCredit, MediaItem, MediaRecommendation, Mix, MixId, Page, Paging, PlaybackInfo, PlaybackInfoOptions, Session, Track, User,
    UserClient, UserSubscription,
  },
  client::auth::{DeviceFlowResponse, GrantType, UserFlowInfo},
  error::{AuthError, UsersError},
  interface::catalogue::playlist::PlaylistCollection,
  utils, Result,
};
use isocountry::CountryCode;
use reqwest::{
  blocking::{Client as ReqwestClient, Response},
  header::HeaderMap,
};
use serde::{Deserialize, Serialize};
use tidal_rs_macros::{
  base_url, basic_auth, bearer_auth, body, body_form_url_encoded, client, delete, get, headers, post, put, query, response_handler, shared_query,
};
use url::Url;
use uuid::Uuid;

pub mod auth;
use auth::{AuthCreds, TokenResponse};

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
  /// get a reference to the `Client`'s auth credentials
  pub fn get_auth_credentials(&self) -> Result<&AuthCreds> {
    self.auth_credentials.as_ref().ok_or(AuthError::Unauthenticated.into())
  }
  /// get a mutable reference to the `Client`'s auth credentials
  pub fn get_auth_credentials_mut(&mut self) -> Result<&mut AuthCreds> {
    self.auth_credentials.as_mut().ok_or(AuthError::Unauthenticated.into())
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
    self.auth_credentials.as_ref().map_or(false, |creds| creds.refresh_token().is_ok())
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
}

///
/// Private methods for http requests and auth handling
///
impl Client {
  fn bearer_auth(&self) -> Option<&str> {
    self.get_auth_credentials().ok().map(|a| a.access_token())
  }
  fn basic_auth(&self) -> (&str, Option<&str>) {
    let creds = self.get_client_credentials().as_tuple();
    (creds.0, Some(creds.1))
  }
  fn shared_query(&self) -> [(&str, &str); 3] {
    let cc = self.get_country_code().map_or("WW", |cc| cc.alpha2());
    [("countryCode", cc), ("locale", "en_US"), ("deviceType", "BROWSER")]
  }
}

///
/// Request methods returning a `Response` object
///
#[client(self.http_client)]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[query(&self.shared_query())]
#[response_handler(|res| {Ok(res)})]
impl Client {
  /// get a page as a `Response`
  #[get(format!("/pages/{page}"))]
  #[base_url("https://api.tidal.com/v1")]
  pub fn get_page_response(&self, page: &str) -> Result<Response> {}
  /// get an endpoint as a `Response`
  #[get(path)]
  #[base_url(base_url)]
  pub fn get_endpoint_response(&self, base_url: &str, path: &str) -> Result<Response> {}
}

///
/// Request methods to the v1 API
///
#[client(self.http_client)]
#[base_url("https://api.tidal.com/v1")]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[shared_query(&self.shared_query())]
impl Client {
  /// Get a session from the current authentication credentials
  #[get("/sessions")]
  pub fn get_session_from_auth(&self) -> Result<Session> {}
  /// Get a session from it's specified id
  #[get(format!("/sessions/{session_id}"))]
  pub fn get_session(&self, session_id: &str) -> Result<Session> {}

  // Users //
  /// Get a user by their user_id
  #[get(format!("/users/{user_id}"))]
  pub fn get_user(&self, user_id: &u64) -> Result<User> {}
  /// Get a user's subscription
  #[get(format!("/users/{user_id}/subscription"))]
  pub fn get_user_subscription(&self, user_id: &u64) -> Result<UserSubscription> {}
  /// Get a user's clients
  #[get(format!("/users/{user_id}/clients"))]
  pub fn get_user_clients(&self, user_id: &u64) -> Result<Paging<UserClient>> {}
  /// Authorize a client for offline access
  #[post(format!("/users/{client_id}/clients"))]
  #[body(&[("clientName", name), ("clientId", &client_id.to_string())])]
  #[body_form_url_encoded]
  pub fn authorize_client(&self, client_id: &u64, name: &str) -> Result<()> {}
  /// Deauthorize a client for offline access
  #[delete(format!("/users/{client_id}/clients"))]
  pub fn deauthorize_client(&self, client_id: &u64) -> Result<()> {}
  /// Get the current user
  pub fn get_current_user(&self) -> Result<User> {
    let credentials = self.get_auth_credentials()?;
    self.get_user(credentials.user_id().ok_or(UsersError::NoCurrentUser)?)
  }
  /// Get the current user's subscription
  pub fn get_current_user_subscription(&self) -> Result<UserSubscription> {
    let credentials = self.get_auth_credentials()?;
    self.get_user_subscription(credentials.user_id().ok_or(UsersError::NoCurrentUser)?)
  }
  /// Get the current user's clients
  pub fn get_current_user_clients(&self) -> Result<Paging<UserClient>> {
    let credentials = self.get_auth_credentials()?;
    self.get_user_clients(credentials.user_id().ok_or(UsersError::NoCurrentUser)?)
  }

  // Catalogue //
  /// Get a given page under the `/pages` endpoint
  #[get(format!("/pages/{page}"))]
  pub fn get_page(&self, page: &str) -> Result<Page> {}
  pub fn get_home_page(&self) -> Result<Page> {
    self.get_page("home")
  }
  pub fn get_explore_page(&self) -> Result<Page> {
    self.get_page("explore")
  }
  pub fn get_mix_page(&self, mix_id: &str) -> Result<Page> {
    let path = format!("mix?mixId={}", mix_id);
    self.get_page(path.as_str())
  }
  pub fn get_artist_page(&self, artist_id: &u64) -> Result<Page> {
    let path = format!("artist?artistId={}", artist_id);
    self.get_page(path.as_str())
  }
  pub fn get_album_page(&self, album_id: &u64) -> Result<Page> {
    let path = format!("album?albumId={}", album_id);
    self.get_page(path.as_str())
  }

  // Catalogue Tracks //
  /// Get a track by its id
  #[get(format!("/tracks/{track_id}"))]
  pub fn get_track(&self, track_id: &u64) -> Result<Track> {}
  /// Get a track's credits
  #[get(format!("/tracks/{track_id}/credits"))]
  #[query(&[
    ("limit", &limit.to_string()), 
    ("includeContributors", &include_contributors.to_string())
  ])]
  pub fn get_track_credits(&self, track_id: &u64, limit: &u64, include_contributors: bool) -> Result<Vec<MediaCredit>> {}
  /// Get a track's mix id
  #[get(format!("/tracks/{track_id}/mix"))]
  pub fn get_track_mix_id(&self, track_id: &u64) -> Result<MixId> {}
  /// Get a track's lyrics
  #[get(format!("/tracks/{track_id}/lyrics"))]
  pub fn get_track_lyrics(&self, track_id: &u64) -> Result<Lyrics> {}
  /// Get a track's recommendations
  #[get(format!("/tracks/{track_id}/recommendations"))]
  #[query(&[("limit", &limit.to_string())])]
  pub fn get_track_recommendations(&self, track_id: &u64, limit: &u64) -> Result<Paging<MediaRecommendation>> {}
  /// Get a track's playback information
  #[get(format!("/tracks/{track_id}/playbackinfo"))]
  #[query(&options.get_query_params())]
  #[headers([
    ("x-tidal-token", self.client_credentials.id()),
    ("x-tidal-prefetch", &options.prefetch.to_string()),
    ("x-tidal-streamingsessionid", &self.streaming_session_id.to_string()),
  ])]
  pub fn playback_info_for_track(&self, track_id: &u64, options: &PlaybackInfoOptions) -> Result<PlaybackInfo> {}

  // Catalogue Videos //
  /// Get a video by its id
  #[get(format!("/videos/{video_id}"))]
  pub fn get_video(&self, video_id: &u64) -> Result<crate::api::Video> {}
  /// Get a video's reccommendations
  #[get(format!("/videos/{video_id}/recommendations"))]
  #[query(&[("limit", &limit.to_string())])]
  pub fn get_video_recommendations(&self, video_id: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaRecommendation>> {}
  /// Get a video's playback information
  #[get(format!("/videos/{video_id}/playbackinfo"))]
  #[query(&options.get_query_params())]
  pub fn playback_info_for_video(&self, video_id: &u64, options: &crate::api::PlaybackInfoOptions) -> Result<crate::api::PlaybackInfo> {}

  // Catalogue Artists //
  /// Get an artist by their id
  #[get(format!("/artists/{artist_id}"))]
  pub fn get_artist(&self, artist_id: &u64) -> Result<crate::api::Artist> {}
  /// Get an artist's bio
  #[get(format!("/artists/{artist_id}/bio"))]
  pub fn get_artist_bio(&self, artist_id: &u64) -> Result<crate::api::ArtistBio> {}
  /// Get an artist's mix id
  #[get(format!("/artists/{artist_id}/mix"))]
  pub fn get_artist_mix_id(&self, artist_id: &u64) -> Result<crate::api::MixId> {}
  /// Get an artist's top tracks
  #[get(format!("/artists/{artist_id}/toptracks"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  pub fn get_artist_top_tracks(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::Track>> {}
  /// Get an artist's videos
  #[get(format!("/artists/{artist_id}/videos"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  pub fn get_artist_videos(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::Video>> {}
  /// Get an artist's albums
  #[get(format!("/artists/{artist_id}/albums"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  pub fn get_artist_albums(&self, artist_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::Album>> {}

  // Catalogue Albums //
  /// Get an album by its id
  #[get(format!("/albums/{album_id}"))]
  pub fn get_album(&self, album_id: &u64) -> Result<crate::api::Album> {}
  /// Get an album's credits
  #[get(format!("/albums/{album_id}/credits"))]
  #[query(&[("includeContributors", &include_contributors.to_string())])]
  pub fn get_album_credits(&self, album_id: &u64, include_contributors: bool) -> Result<Vec<crate::api::MediaCredit>> {}
  /// Get an album's media items
  #[get(format!("/albums/{album_id}/items"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  pub fn get_album_items(&self, album_id: &u64, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaItem>> {}
  /// Get an album's media items with their credits
  #[get(format!("/albums/{album_id}/items/credits"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
    ("includeContributors", &include_contributors.to_string())
  ])]
  pub fn get_album_items_with_credits(
    &self,
    album_id: &u64,
    offset: &u64,
    limit: &u64,
    include_contributors: bool,
  ) -> Result<crate::api::Paging<crate::api::MediaItem>> {
  }

  // Catalogue Playlists //
  /// Get a playlist by its id
  #[get(format!("/playlists/{playlist_id}"))]
  #[response_handler(|res: Response| -> Result<crate::api::Playlist> {
    let etag = res.headers().get("ETag").map(|v| v.to_str().unwrap().to_string());
    let mut playlist: crate::api::Playlist = res.json()?;
    playlist.etag = etag;
    Ok(playlist)
  })]
  pub fn get_playlist(&self, playlist_id: &Uuid) -> Result<crate::api::Playlist> {}
  /// Get a playlist's items
  #[get(format!("/playlists/{playlist_id}/items"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  pub fn get_playlist_items(&self, playlist_id: &Uuid, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaItem>> {}
  /// Get a playlist's recommendations
  #[get(format!("/playlists/{playlist_id}/recommendations/items"))]
  #[query(&[
    ("offset", &offset.to_string()),
    ("limit", &limit.to_string()), 
  ])]
  pub fn get_playlist_recommendations(&self, playlist_id: &Uuid, offset: &u64, limit: &u64) -> Result<crate::api::Paging<crate::api::MediaItem>> {}

  // Catalogue Mixes //
  /// Get a mix by its id
  #[get(format!("/mixes/{mix_id}/items"))]
  #[query(&[("offset", offset.to_string()), ("limit", limit.to_string())])]
  pub fn get_mix_items(&self, mix_id: &str, offset: &u64, limit: &u64) -> Result<Paging<MediaItem>> {}
}

///
/// Request methods to the v2 API
///
#[client(self.http_client)]
#[base_url("https://api.tidal.com/v2")]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[shared_query(&self.shared_query())]
#[response_handler(|res| {Ok(res)})]
impl Client {
  #[put("/my-collection/playlists/folders/add-favorites")]
  #[query(&[
    ("folderId", parent_folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()})),
    ("uuids", playlist_id.to_string())
  ])]
  fn add_favorite_playlist(&self, parent_folder_id: Option<&Uuid>, playlist_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/create-playlist")]
  #[query(&[
    ("name", name),
    ("description", description),
    ("folderId", &parent_folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()})),
    ("isPublic", &is_public.to_string())
  ])]
  fn create_playlist(&self, parent_folder_id: Option<&Uuid>, name: &str, description: &str, is_public: bool) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/remove")]
  #[query(&[("trns", format!("trn:playlist:{playlist_id}"))])]
  fn remove_playlist(&self, playlist_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/move")]
  #[query(&[
    ("folderId", parent_folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()})),
    ("trns", format!("trn:playlist:{playlist_id}"))
  ])]
  fn move_playlist(&self, parent_folder_id: Option<&Uuid>, playlist_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/rename")]
  #[query(&[
    ("name", name),
    ("description", description),
    ("trn", &format!("trn:playlist:{playlist_id}")),
  ])]
  fn edit_playlist(&self, playlist_id: &Uuid, name: &str, description: &str) -> Result<Response> {}

  #[put(format!("/playlists/{playlist_id}/set-public"))]
  fn publish_playlist(&self, playlist_id: &Uuid) -> Result<Response> {}

  #[put(format!("/playlists/{playlist_id}/set-private"))]
  fn unpublish_playlist(&self, playlist_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/items")]
  #[query(&[("trns", format!("trn:playlist:{playlist_id}"))])]
  fn get_collection_playlist(&self, playlist_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/create-folder")]
  #[query(&[
    ("name", name),
    ("folderId", &parent_folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()})),
    ("trns", _trns.unwrap_or_default())
  ])]
  fn create_folder(&self, parent_folder_id: Option<&Uuid>, name: &str, _trns: Option<&str>) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/remove")]
  #[query(&[("trns", format!("trn:folder:{folder_id}"))])]
  fn remove_folder(&self, folder_id: &Uuid) -> Result<Response> {}

  #[get("/my-collection/playlists/folders")]
  #[query(&[("folderId", folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()}))])]
  fn get_folder_items(&self, folder_id: Option<&Uuid>) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/move")]
  #[query(&[
    ("folderId", parent_folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()})),
    ("trns", format!("trn:folder:{folder_id}"))
  ])]
  fn move_folder(&self, parent_folder_id: Option<&Uuid>, folder_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/rename")]
  #[query(&[
    ("name", name),
    ("trn", &format!("trn:folder:{folder_id}")),
  ])]
  fn rename_folder(&self, folder_id: &Uuid, name: &str) -> Result<Response> {}
}

///
/// Methods for auth
///
/// # Client Flow
///
/// Client flow uses only the client credentials to authenticate, offering only basic access
///
/// # User Flow
///
/// User flow works in two stages, first `user_flow_login_init()` is called, which should return a
/// properly structured URL and a PKCE verifier. The user should then visit the URL and authorize
/// the application. Once the user has authorized the application, they will be redirected to a
/// specified redirect URI with a code as a query parameter. This code, along with the PKCE
/// verifier should be passed to `user_flow_login_finalize()`, which will complete the login process.
///
/// # Device Flow
///
/// Device flow is similar to user flow, but is designed for devices with limited input
/// capabilities. The device flow is initiated with `device_flow_login_init()`, which should return a
/// user code and a verification URI to be displayed to the user, as well as a device code, expiry
/// time, and polling interval, which will be used when finalizing authentication. The user should
/// visit the verification URI, enter the user code, and authenticate the application. Once the
/// user has authenticated the application, `try_device_flow_login_finalize()` can be called to complete
/// the login process. Since device flow does not require direct user input, it is possible, and
/// recomended to call `device_flow_login_finalize()` instead, which will automatically poll the server
/// until either the user has authenticated the application, or the flow has expired.

#[client(self.http_client)]
#[base_url("https://auth.tidal.com/v1")]
#[basic_auth(self.basic_auth())]
#[shared_query(&self.shared_query())]
impl Client {
  // Client Flow //
  /// Authenticate using only the client credentials
  #[post("/oauth2/token")]
  #[body_form_url_encoded]
  #[body(&[("grant_type", "client_credentials")])]
  #[response_handler(|res: Response| {
      if !res.status().is_success() {
        println!("Error: {}", res.status());
        println!("Response: {}", res.text().unwrap_or_default());
        panic!();
        // return Err(utils::res_to_error(res));
      }
      let auth_creds = AuthCreds::new(GrantType::ClientCredentials, self.client_credentials.clone(), res.json::<TokenResponse>()?);
      let country = auth_creds.auth_user().map(|user| user.country_code);

      self.auth_credentials = Some(auth_creds);
      self.country = country;
      Ok(())
  })]
  pub fn client_flow_login(&mut self) -> Result<()> {}

  // User Flow //
  /// Returns a `UserFlowInfo` containing a URL for the user to authorize the application, and a
  /// PKCE verifier to be used in conjunction with the code returned from the redirect URI.
  pub fn user_flow_login_init(&self) -> Result<UserFlowInfo> {
    let redirect_uri = self.redirect_uri.as_ref().ok_or(AuthError::Unauthenticated)?;
    let scopes = self.scopes.join(" ");
    let (pkce_challenge, pkce_verifier) = utils::new_pkce_pair();

    let auth_url = Url::parse_with_params(
      "https://login.tidal.com/authorize",
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
  /// Consumes both the `UserFlowInfo` from `user_flow_login_init()` and the code returned from the
  /// redirect URI to finalize the user flow login.
  #[post("/oauth2/token")]
  #[body_form_url_encoded]
  #[body(&[
    ("code_verifier", info.verifier()),
    ("code", &code),
    ("client_id", self.client_credentials.id()),
    ("redirect_uri", &self.redirect_uri.as_ref().ok_or(AuthError::MissingRedirectUri)?.clone()),
    ("grant_type", GrantType::AuthorizationCode.as_str()),
  ])]
  #[response_handler(|res: Response| {
    if !res.status().is_success() {
      return Err(utils::res_to_error(res));
    }
    let auth_creds = AuthCreds::new(GrantType::AuthorizationCode, self.client_credentials.clone(), res.json::<TokenResponse>()?);
    let country = auth_creds.auth_user().map(|user| user.country_code);

    self.auth_credentials = Some(auth_creds);
    self.country = country;
    Ok(())
  })]
  pub fn user_flow_login_finalize(&mut self, code: String, info: UserFlowInfo) -> Result<()> {}

  // Device Flow //
  /// Initializes the device flow login process, returning a `DeviceFlowResponse` containing the user code,
  /// verification URI, device code, expiry time, and polling interval.
  #[post("/oauth2/device_authorization")]
  #[body_form_url_encoded]
  #[body(&[
    ("scope", "r_usr+w_usr+w_sub"), 
    ("client_id", self.get_client_id())
  ])]
  #[response_handler(|res: Response| {
    if !res.status().is_success() {
      return Err(utils::res_to_error(res));
    }
    let response = res.json::<DeviceFlowResponse>()?;
    Ok(response)
  })]
  pub fn device_flow_login_init(&self) -> Result<DeviceFlowResponse> {}
  /// Attempts to finalize the device flow login process, consuming the `DeviceFlowResponse` from `device_flow_login_init()`.
  #[post("/oauth2/token")]
  #[body_form_url_encoded]
  #[body(&[
    ("scope", "r_usr+w_usr+w_sub"),
    ("client_id", self.client_credentials.id()),
    ("device_code", &response.device_code),
    ("grant_type", GrantType::DeviceCode.as_str()),
  ])]
  #[response_handler(|res: Response| {
    if !res.status().is_success() {
      return Err(utils::res_to_error(res));
    }
    let auth_creds = AuthCreds::new(GrantType::DeviceCode, self.client_credentials.clone(), res.json::<TokenResponse>()?);
    let country = auth_creds.auth_user().map(|user| user.country_code);

    self.auth_credentials = Some(auth_creds);
    self.country = country;
    Ok(())
  })]
  pub fn try_device_flow_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()> {}
  /// Attempts to finalize the device flow login process, retrying until either the user has authenticated,
  /// the flow has expired, or the maximum number of retries has been reached.
  pub fn device_flow_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()> {
    let interval = response.interval;
    let max_retries = response.expires_in / interval;

    let mut i: i64 = 0;
    while i < max_retries {
      match self.try_device_flow_login_finalize(response) {
        Err(crate::Error::AuthError(AuthError::AuthorizationPending)) => {
          i += 1;
          std::thread::sleep(std::time::Duration::from_secs(interval as u64));
        }
        res => return res,
      }
    }
    Err(AuthError::MaxRetriesReached)?
  }

  // Refresh Flow //
  /// Refreshes the current credentials
  pub fn refresh(&mut self) -> Result<()> {
    self
      .auth_credentials
      .as_mut()
      .ok_or(AuthError::Unauthenticated)?
      .refresh_with_http_client(&self.http_client)
  }
}
#[client(self.http_client)]
#[base_url("https://api.tidal.com/v2")]
#[bearer_auth(self.bearer_auth().unwrap_or_default())]
#[shared_query(&self.shared_query())]
#[response_handler(|res| {Ok(res)})]
impl PlaylistCollection for Client {
  #[put("/my-collection/playlists/folders/add-favorites")]
  #[query(&[
    ("folderId", parent_folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()})),
    ("uuids", playlist_id.to_string())
  ])]
  fn add_favorite_playlist(&self, parent_folder_id: Option<&Uuid>, playlist_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/create-playlist")]
  #[query(&[
    ("name", name),
    ("description", description),
    ("folderId", &parent_folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()})),
    ("isPublic", &is_public.to_string())
  ])]
  fn create_playlist(&self, parent_folder_id: Option<&Uuid>, name: &str, description: &str, is_public: bool) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/remove")]
  #[query(&[("trns", format!("trn:playlist:{playlist_id}"))])]
  fn remove_playlist(&self, playlist_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/move")]
  #[query(&[
    ("folderId", parent_folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()})),
    ("trns", format!("trn:playlist:{playlist_id}"))
  ])]
  fn move_playlist(&self, parent_folder_id: Option<&Uuid>, playlist_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/rename")]
  #[query(&[
    ("name", name),
    ("description", description),
    ("trn", &format!("trn:playlist:{playlist_id}")),
  ])]
  fn edit_playlist(&self, playlist_id: &Uuid, name: &str, description: &str) -> Result<Response> {}

  #[put(format!("/playlists/{playlist_id}/set-public"))]
  fn publish_playlist(&self, playlist_id: &Uuid) -> Result<Response> {}

  #[put(format!("/playlists/{playlist_id}/set-private"))]
  fn unpublish_playlist(&self, playlist_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/items")]
  #[query(&[("trns", format!("trn:playlist:{playlist_id}"))])]
  fn get_collection_playlist(&self, playlist_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/create-folder")]
  #[query(&[
    ("name", name),
    ("folderId", &parent_folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()})),
    ("trns", _trns.unwrap_or_default())
  ])]
  fn create_folder(&self, parent_folder_id: Option<&Uuid>, name: &str, _trns: Option<&str>) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/remove")]
  #[query(&[("trns", format!("trn:folder:{folder_id}"))])]
  fn remove_folder(&self, folder_id: &Uuid) -> Result<Response> {}

  #[get("/my-collection/playlists/folders")]
  #[query(&[("folderId", folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()}))])]
  fn get_folder_items(&self, folder_id: Option<&Uuid>) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/move")]
  #[query(&[
    ("folderId", parent_folder_id.map_or("root".to_owned(), |uuid| {uuid.to_string()})),
    ("trns", format!("trn:folder:{folder_id}"))
  ])]
  fn move_folder(&self, parent_folder_id: Option<&Uuid>, folder_id: &Uuid) -> Result<Response> {}

  #[put("/my-collection/playlists/folders/rename")]
  #[query(&[
    ("name", name),
    ("trn", &format!("trn:folder:{folder_id}")),
  ])]
  fn rename_folder(&self, folder_id: &Uuid, name: &str) -> Result<Response> {}
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
