use std::fmt::Debug;
use crate::{client::ClientCreds, error::ApiErrorResponse, Error, Result};
use serde::Deserialize;

mod oauth;
pub mod flows;
pub use flows::{ClientFlow, UserFlow, DeviceFlow, RefreshFlow};
pub mod credentials;
pub use credentials::Credentials;

#[derive(Deserialize)]
pub struct TokenResponse {
  access_token: String,
  scope: String,
  expires_in: u64,
  #[serde(default)]
  user_id: Option<u64>,
  #[serde(default)]
  refresh_token: Option<String>,
}
impl Debug for TokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenResponse").field("access_token", &"[Redacted]").field("user_id", &self.user_id).field("scope", &self.scope).field("expires_in", &self.expires_in).field("refresh_token", &"[Redacted]").finish()
    }
}

pub struct AuthClient {
  client_credentials: ClientCreds,
  /// Authorisation Configuration
  redirect_uri: Option<String>,
  /// Credentials for the current session
  credentials: Option<Credentials>,
}
impl AuthClient {
  pub fn new(client_credentials: ClientCreds) -> Self { Self {
    client_credentials: client_credentials,
    redirect_uri: None,
    credentials: None,
  }}

  pub fn get_credentials(&mut self) -> Result<&Credentials> {
    let credentials = self.credentials.as_mut().ok_or(AuthError::Unauthenticated)?;
    let expire_time = credentials.expires_at();
    let cur_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    if expire_time <= cur_time {
      credentials.refresh()?;
    }
    Ok(credentials)
  }

  pub fn set_redirect_uri(&mut self, redirect_uri: String) {
    self.redirect_uri = Some(redirect_uri);
  }
}

pub trait Auth {
  fn get_credentials(&self) -> Result<&Credentials>;
  fn get_credentials_mut(&mut self) -> Result<&mut Credentials>;

  fn get_credentials_refresh(&mut self) -> Result<&Credentials> {
    self.credentials_refresh()?;
    Ok(self.get_credentials()?)
  }
  fn get_credentials_force_refresh(&mut self) -> Result<&Credentials> {
    self.credentials_force_refresh()?;
    Ok(self.get_credentials()?)
  }
  fn credentials_refresh(&mut self) -> Result<()> {
    let credentials = self.get_credentials_mut()?;
    let expire_time = credentials.expires_at();
    let cur_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    if expire_time <= cur_time {
      credentials.refresh()?;
    }
    Ok(())
  }
  fn credentials_force_refresh(&mut self) -> Result<()> {
    self.get_credentials_mut()?.refresh()
  }
}

#[derive(Debug)]
pub enum AuthError {
  InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
  ReqwestError(reqwest::Error),
  ApiError(ApiErrorResponse),
  AuthorizationPending,
  MissingRedirectUri,
  MaxRetriesReached,
  Unauthenticated,
}
impl From<reqwest::header::InvalidHeaderValue> for Error {
  fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
    Error::AuthError(AuthError::InvalidHeaderValue(err))
  }
}
impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
    Error::AuthError(AuthError::ReqwestError(err))
  }
}
