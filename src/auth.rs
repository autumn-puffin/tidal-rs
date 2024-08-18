use std::{cell::RefCell, fmt::Debug, rc::Rc};
use crate::{client::ClientCreds, error::ApiErrorResponse, Result};
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

pub struct Auth {
  client_credentials: Rc<ClientCreds>,
  /// Authorisation Configuration
  redirect_uri: Option<String>,
  /// Credentials for the current session
  credentials: Option<Rc<RefCell<Credentials>>>,
  
}
impl Auth {
  pub fn new(client_credentials: ClientCreds, redirect_uri: Option<String>) -> Self { Self {
    client_credentials: Rc::new(client_credentials),
    redirect_uri,
    credentials: None,
  }}

  pub fn get_credentials(&mut self) -> Result<Rc<RefCell<Credentials>>> {
    let credentials = self.credentials.clone().ok_or(AuthError::Unauthenticated)?;
    let expire_time = credentials.borrow().expires_at();
    let cur_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    if expire_time <= cur_time {
      self.refresh()?;
    }
    Ok(credentials)
  }

  pub fn set_redirect_uri(&mut self, redirect_uri: String) {
    self.redirect_uri = Some(redirect_uri);
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
impl From<reqwest::header::InvalidHeaderValue> for AuthError {
  fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
    AuthError::InvalidHeaderValue(err)
  }
}
impl From<reqwest::Error> for AuthError {
  fn from(err: reqwest::Error) -> Self {
    AuthError::ReqwestError(err)
  }
}
