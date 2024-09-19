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
  api::{Page, Paging, PlaybackInfo, PlaybackInfoOptions, Session, Track, User, UserClient, UserSubscription},
  endpoints::Endpoint,
  error::ApiErrorResponse,
  utils::{self, get_request_helper, oauth_request_helper, post_request_helper},
  Result,
};
use isocountry::CountryCode;
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
  pub fn as_auth(&self) -> AuthClient {
    AuthClient::new(self.client_credentials.clone())
  }
  pub fn as_catalogue(&self) -> CatalogueClient {
    CatalogueClient::new(self.client_credentials.clone())
  }
  pub fn set_redirect_uri(&mut self, uri: String) {
    self.redirect_uri = Some(uri);
  }
  pub fn set_country(&mut self, country: CountryCode) {
    self.country = Some(country);
  }

  pub fn get_page_response(&self, page: &str) -> Result<Response> {
    let endpoint = Endpoint::Pages(page);
    let auth = self.get_credentials()?;

    get_request_helper(&self.http_client, endpoint, auth)
      .query(&[("countryCode", self.get_country()?.alpha2()), ("deviceType", "BROWSER")])
      .send()
      .map_err(Into::into)
  }
}
impl Client {
  fn oauth_helper(&mut self, endpoint: Endpoint, grant: GrantType, params: Option<&[(&str, &str)]>) -> Result<()> {
    let client_creds = &self.client_credentials;
    let res = oauth_request_helper(&self.http_client, endpoint, grant, client_creds, params).send()?;
    if !res.status().is_success() {
      return Err(res.json::<ApiErrorResponse>()?.into());
    }
    let auth_creds = AuthCreds::new(grant, self.client_credentials.clone(), res.json::<TokenResponse>()?);
    let country = auth_creds.auth_user().map(|user| user.country_code);

    self.auth_credentials = Some(auth_creds);
    self.country = country;
    Ok(())
  }
  fn get_helper(
    &self,
    endpoint: Endpoint,
    query: Option<&[(&str, &str)]>,
    form: Option<&[(&str, &str)]>,
    headers: Option<&[(&str, &str)]>,
  ) -> Result<Response> {
    let auth = self.get_credentials()?;
    let headers = &HashMap::from_iter(headers.unwrap_or_default().iter().map(|(k, v)| (k.to_string(), v.to_string())));
    let headers = HeaderMap::try_from(headers).unwrap();
    self
      .http_client
      .get(endpoint.to_string())
      .query(query.unwrap_or_default())
      .form(form.unwrap_or_default())
      .headers(headers)
      .bearer_auth(auth.access_token())
      .send()
      .map_err(Into::into)
  }
  fn post_helper(
    &self,
    endpoint: Endpoint,
    query: Option<&[(&str, &str)]>,
    form: Option<&[(&str, &str)]>,
    headers: Option<&[(&str, &str)]>,
  ) -> Result<Response> {
    let auth = self.get_credentials()?;
    let headers = &HashMap::from_iter(headers.unwrap_or_default().iter().map(|(k, v)| (k.to_string(), v.to_string())));
    let headers = HeaderMap::try_from(headers).unwrap();
    self
      .http_client
      .post(endpoint.to_string())
      .query(query.unwrap_or_default())
      .form(form.unwrap_or_default())
      .headers(headers)
      .bearer_auth(auth.access_token())
      .send()
      .map_err(Into::into)
  }
  fn delete_helper(
    &self,
    endpoint: Endpoint,
    query: Option<&[(&str, &str)]>,
    form: Option<&[(&str, &str)]>,
    headers: Option<&[(&str, &str)]>,
  ) -> Result<Response> {
    let auth = self.get_credentials()?;
    let headers = &HashMap::from_iter(headers.unwrap_or_default().iter().map(|(k, v)| (k.to_string(), v.to_string())));
    let headers = HeaderMap::try_from(headers).unwrap();
    self
      .http_client
      .delete(endpoint.to_string())
      .query(query.unwrap_or_default())
      .form(form.unwrap_or_default())
      .headers(headers)
      .bearer_auth(auth.access_token())
      .send()
      .map_err(Into::into)
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

    Ok(UserFlowInfo { auth_url, pkce_verifier })
  }

  fn user_login_finalize(&mut self, code: String, info: UserFlowInfo) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::AuthorizationCode;
    let verifier = info.pkce_verifier.as_str();
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
    let client_credentials = &self.client_credentials;

    let params = &[("scope", "r_usr+w_usr+w_sub"), ("client_id", self.client_credentials.id())];

    let res = post_request_helper(&self.http_client, endpoint, client_credentials).form(params).send()?;
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
    let query = &[("CountryCode", self.country.unwrap_or(CountryCode::USA).alpha2())];
    let res = self.get_helper(endpoint, Some(query), None, None)?;
    Ok(res.json()?)
  }

  fn get_user_subscription(&self, user_id: &u64) -> Result<UserSubscription> {
    let endpoint = Endpoint::UsersSubscription(user_id);
    let query = &[("CountryCode", self.country.unwrap_or(CountryCode::USA).alpha2())];
    let res = self.get_helper(endpoint, Some(query), None, None)?;
    Ok(res.json()?)
  }

  fn get_user_clients(&self, user_id: &u64) -> Result<Paging<UserClient>> {
    let endpoint = Endpoint::UsersClients(user_id);
    let query = &[("CountryCode", self.country.unwrap_or(CountryCode::USA).alpha2())];
    let res = self.get_helper(endpoint, Some(query), None, None)?;
    Ok(res.json()?)
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
    let query = &[("CountryCode", self.country.unwrap_or(CountryCode::USA).alpha2())];
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

/// A simple struct for storing client credentials, with a custom Debug impl that redacts the client secret.
#[derive(Clone, Serialize, Deserialize)]
pub struct ClientCreds {
  client_id: String,
  client_secret: String,
}
impl ClientCreds {
  pub fn new(client_id: String, client_secret: String) -> Self {
    Self { client_id, client_secret }
  }
  pub fn id(&self) -> &str {
    &self.client_id
  }
  pub fn secret(&self) -> &str {
    &self.client_secret
  }
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
