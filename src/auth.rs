use std::{collections::HashMap, fmt::Debug, rc::Rc};
use crate::{client::ClientCreds, error::ApiErrorResponse, Result};
use base64::prelude::*;
use reqwest::{blocking::Client, header::HeaderMap};
use serde::Deserialize;

mod oauth;
pub mod flows;
pub use flows::{ClientFlow, UserFlow, DeviceFlow};
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
  credentials: Option<Credentials>,
  
}
impl Auth {
  pub fn new(client_credentials: ClientCreds, redirect_uri: Option<String>) -> Self { Self {
    client_credentials: Rc::new(client_credentials),
    redirect_uri,
    credentials: None,
  }}
  
  pub fn client(&self) -> Result<Client> {
    let client_creds = &self.client_credentials;
    let mut headers = HeaderMap::new();
    let header_auth = format!("Basic {}", BASE64_STANDARD.encode(format!("{}:{}", client_creds.id(), client_creds.secret()))); 
    headers.insert("Authorization", header_auth.parse()?); // TODO: use better error 

    Client::builder().default_headers(headers).build().map_err(Into::into)
  } 

  pub fn refresh_creds(&mut self) -> Result<()> {
    let client = self.client()?;
    let creds = self.credentials.as_mut().ok_or(AuthError::Unauthenticated)?;
    
    if let Some(refresh_token) = creds.refresh_token() {
      let mut params = HashMap::new();
      params.insert("grant_type", "refresh_token");
      params.insert("refresh_token", refresh_token);

      let res = client
        .post("https://auth.tidal.com/v1/oauth2/token")
        .form(&params).send()?;

      if res.status().is_success() {
        self.credentials = Some(res.json::<TokenResponse>()?.into());
      } else {
        let err = res.json::<ApiErrorResponse>()?;
        Err(AuthError::ApiError(err))?
      }
      Ok(())
    } else {
      self.client_login()
    }
  }

  pub fn get_credentials(&mut self) -> Result<&Credentials> {
    let expire_time = self.credentials.as_ref().ok_or(AuthError::Unauthenticated)?.expires_at();
    let cur_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    if expire_time <= cur_time {
      self.refresh_creds()?;
    }
    Ok(self.credentials.as_ref().unwrap())
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
