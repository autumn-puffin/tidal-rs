//! TIDAL-rs' main client implementation for the Tidal API, implimenting all of the available interfaces.
//!
//! The `Client` struct is a basic all-inclusive blocking client for the Tidal API, there are also
//! standalone clients for individual parts of the API, such as the `AuthClient`, and the `CatalogueClient`

pub use crate::interface::{
  auth::{flows::*, *},
  catalogue::*,
  users::*,
};
use crate::{
  api::{Page, Paging, User, UserClient, UserSubscription},
  endpoints::Endpoint,
  error::ApiErrorResponse,
  utils::{self, get_request_helper, oauth_request_helper, post_request_helper},
  Result,
};
use isocountry::CountryCode;
use reqwest::blocking::{Client as ReqwestClient, Response};
use std::collections::HashMap;
use url::Url;

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
}
impl Client {
  pub fn new(client_credentials: ClientCreds) -> Self {
    let credentials = client_credentials;

    Self {
      http_client: ReqwestClient::new(),
      client_credentials: credentials,
      auth_credentials: None,
      scopes: Vec::new(),
      redirect_uri: None,
      country: None,
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

  fn get_page_response(&self, page: &str) -> Result<Response> {
    let endpoint = Endpoint::Pages(page);
    let auth = self.get_credentials()?;

    get_request_helper(&self.http_client, endpoint, auth)
      .query(&[("countryCode", self.get_country()?.alpha2()), ("deviceType", "BROWSER")])
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
impl ClientFlow for Client {
  fn client_login(&mut self) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::ClientCredentials;

    let res = oauth_request_helper(&self.http_client, endpoint, grant, &self.client_credentials, None).send()?;

    self.auth_credentials = Some(AuthCreds::new(grant, self.client_credentials.clone(), res.json::<TokenResponse>()?));
    Ok(())
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
    let client_credentials = &self.client_credentials;
    let redirect_uri = self.redirect_uri.as_ref().ok_or(AuthError::MissingRedirectUri)?;
    let verifier = info.pkce_verifier;
    let params = HashMap::from([
      ("client_id", self.client_credentials.id()),
      ("code", &code),
      ("redirect_uri", redirect_uri),
      ("code_verifier", &verifier),
    ]);
    let res = oauth_request_helper(&self.http_client, endpoint, grant, client_credentials, Some(params)).send()?;
    if !res.status().is_success() {
      return Err(res.json::<ApiErrorResponse>()?.into());
    }

    let credentials = AuthCreds::new(grant, client_credentials.clone(), res.json::<TokenResponse>()?);
    self.country = credentials.auth_user().map(|user| user.country_code);
    self.auth_credentials = Some(credentials);
    Ok(())
  }
}
impl DeviceFlow for Client {
  fn device_login_init(&self) -> Result<crate::interface::auth::flows::DeviceFlowResponse> {
    let endpoint = Endpoint::OAuth2DeviceAuth;
    let client_credentials = &self.client_credentials;

    let params = HashMap::from([("scope", "r_usr+w_usr+w_sub"), ("client_id", self.client_credentials.id())]);

    let res = post_request_helper(&self.http_client, endpoint, client_credentials)
      .form(&params)
      .send()?;
    Ok(res.json()?)
  }

  fn try_device_login_finalize(&mut self, response: &crate::interface::auth::flows::DeviceFlowResponse) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::DeviceCode;
    let client_credentials = &self.client_credentials;
    let params = HashMap::from([
      ("scope", "r_usr+w_usr+w_sub"),
      ("client_id", client_credentials.id()),
      ("device_code", &response.device_code),
    ]);
    let res = oauth_request_helper(&self.http_client, endpoint, grant, client_credentials, Some(params)).send()?;
    if !res.status().is_success() {
      return Err(res.json::<ApiErrorResponse>()?.into());
    }

    let credentials = AuthCreds::new(grant, client_credentials.clone(), res.json::<TokenResponse>()?);
    self.country = credentials.auth_user().map(|user| user.country_code);
    self.auth_credentials = Some(credentials);
    Ok(())
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
    let auth = self.get_credentials()?;

    let res = get_request_helper(&self.http_client, endpoint, auth)
      .query(&[("CountryCode", self.country.unwrap().to_string())])
      .send()?;
    Ok(res.json()?)
  }

  fn get_user_subscription(&self, user_id: &u64) -> Result<UserSubscription> {
    let endpoint = Endpoint::UsersSubscription(user_id);
    let auth = self.get_credentials()?;

    let res = get_request_helper(&self.http_client, endpoint, auth)
      .query(&[("CountryCode", self.country.unwrap().to_string())])
      .send()?;
    Ok(res.json()?)
  }

  fn get_user_clients(&self, user_id: &u64) -> Result<Paging<UserClient>> {
    let endpoint = Endpoint::UsersClients(user_id);
    let auth = self.get_credentials()?;

    let res = get_request_helper(&self.http_client, endpoint, auth)
      .query(&[("CountryCode", self.country.unwrap().to_string())])
      .send()?;
    Ok(res.json()?)
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

/// A simple struct for storing client credentials, with a custom Debug impl that redacts the client secret.
#[derive(Clone)]
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
