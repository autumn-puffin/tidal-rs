//! Authentication interface for the TIDAL API

pub use crate::interface::auth::{flows::*, *};
use crate::{
  client::ClientCreds,
  endpoints::Endpoint,
  error::ApiErrorResponse,
  utils::{self, client_login_impl, oauth_request_helper, post_request_helper},
  Result,
};
use chrono::Utc;
use isocountry::CountryCode;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod credentials;
pub use credentials::AuthCreds;

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
  access_token: String,
  scope: String,
  expires_in: i64,
  #[serde(default)]
  user_id: Option<u64>,
  user: Option<AuthUser>,
  #[serde(default)]
  refresh_token: Option<String>,
}
impl Debug for TokenResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("TokenResponse")
      .field("access_token", &"[Redacted]")
      .field("user_id", &self.user_id)
      .field("user", &self.user)
      .field("scope", &self.scope)
      .field("expires_in", &self.expires_in)
      .field("refresh_token", &"[Redacted]")
      .finish()
  }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthUser {
  pub user_id: u64,
  pub email: String,
  pub country_code: CountryCode,
  pub full_name: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub nickname: Option<String>,
  pub username: String,
  pub address: Option<String>,
  pub city: Option<String>,
  pub postal_code: Option<String>,
  pub us_state: Option<String>,
  pub phone_number: Option<String>,
  pub birthday: Option<u64>,
  pub channel_id: u64,
  pub parent_id: u64,
  #[serde(rename = "acceptedEULA")]
  pub accepted_eula: bool,
  pub created: u64,
  pub updated: u64,
  pub facebook_uid: u64,
  pub apple_uid: Option<u64>,
  pub google_uid: Option<u64>,
  pub account_link_created: bool,
  pub email_verified: bool,
  pub new_user: bool,
}

#[deprecated]
pub struct AuthClient {
  client_credentials: ClientCreds,
  /// Authorisation Configuration
  redirect_uri: Option<String>,
  /// Credentials for the current session
  credentials: Option<AuthCreds>,
}
impl AuthClient {
  pub fn new(client_credentials: ClientCreds) -> Self {
    Self {
      client_credentials,
      redirect_uri: None,
      credentials: None,
    }
  }

  pub fn get_credentials(&mut self) -> Result<&AuthCreds> {
    let credentials = self.credentials.as_mut().ok_or(AuthError::Unauthenticated)?;
    let expire_time = credentials.expires_at();
    let cur_time = Utc::now().timestamp();
    if expire_time <= cur_time {
      credentials.refresh()?;
    }
    Ok(credentials)
  }

  pub fn set_redirect_uri(&mut self, redirect_uri: String) {
    self.redirect_uri = Some(redirect_uri);
  }
}
impl ClientFlow for AuthClient {
  fn client_login(&mut self) -> Result<()> {
    self.credentials = Some(client_login_impl(&Client::new(), &self.client_credentials)?);
    Ok(())
  }
}
impl UserFlow for AuthClient {
  fn user_login_init(&self) -> Result<UserFlowInfo> {
    let redirect_uri = self.redirect_uri.as_deref().ok_or(AuthError::MissingRedirectUri)?;
    let scopes = ["user.read".to_string()]; // TODO: make this configurable

    let (pkce_challenge, pkce_verifier) = utils::new_pkce_pair();
    let auth_url = format!(
      "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&code_challenge_method=S256&code_challenge={}",
      Endpoint::LoginAuthorize,
      self.client_credentials.id(),
      &redirect_uri,
      scopes.join("+"),
      pkce_challenge
    );

    Ok(UserFlowInfo::new(auth_url, pkce_verifier))
  }
  fn user_login_finalize(&mut self, code: String, info: UserFlowInfo) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::AuthorizationCode;
    let client_credentials = &self.client_credentials;

    let redirect_uri = self.redirect_uri.as_deref().ok_or(AuthError::MissingRedirectUri)?;
    let verifier = info.verifier();

    let params = &[
      ("client_id", self.client_credentials.id()),
      ("code", &code),
      ("redirect_uri", redirect_uri),
      ("code_verifier", verifier),
    ];

    let res = oauth_request_helper(&Client::new(), endpoint, grant, client_credentials, Some(params)).send()?;

    let credentials = AuthCreds::new(grant, client_credentials.clone(), res.json::<TokenResponse>()?);
    self.credentials = Some(credentials);
    Ok(())
  }
}
impl DeviceFlow for AuthClient {
  fn device_login_init(&self) -> Result<DeviceFlowResponse> {
    let client = Client::new();
    let endpoint = Endpoint::OAuth2DeviceAuth;
    let client_credentials = &self.client_credentials;

    let params = &[("scope", "r_usr+w_usr+w_sub"), ("client_id", self.client_credentials.id())];

    let res = post_request_helper(&client, endpoint, client_credentials).form(&params).send()?;

    Ok(res.json()?)
  }
  fn try_device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::DeviceCode;
    let client_credentials = &self.client_credentials;

    let params = &[
      ("scope", "r_usr+w_usr+w_sub"),
      ("client_id", self.client_credentials.id()),
      ("device_code", &response.device_code),
    ];

    let res = oauth_request_helper(&Client::new(), endpoint, grant, client_credentials, Some(params)).send()?;

    if res.status().is_success() {
      let credentials = AuthCreds::new(GrantType::DeviceCode, client_credentials.clone(), res.json::<TokenResponse>()?);
      self.credentials = Some(credentials);
      Ok(())
    } else {
      Err(res.json::<ApiErrorResponse>()?.into())
    }
  }
}
impl RefreshFlow for AuthClient {
  fn refresh(&mut self) -> Result<()> {
    self.credentials.as_mut().ok_or(AuthError::Unauthenticated)?.refresh()
  }
}
