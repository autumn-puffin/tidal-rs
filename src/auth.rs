use std::{collections::HashMap, fmt::Debug};
use crate::{error::ApiErrorResponse, Result};
use base64::prelude::*;
use reqwest::{blocking::Client, header::HeaderMap};
use serde::Deserialize;

mod oauth;
pub mod flows;
pub use flows::{ClientFlow, UserFlow, DeviceFlow};


#[derive(Deserialize)]
pub struct TokenResponse {
  access_token: String,
  user_id: u64,
  scope: String,
  expires_in: u64,
  refresh_token: String,
}
impl Debug for TokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenResponse").field("access_token", &"[Redacted]").field("user_id", &self.user_id).field("scope", &self.scope).field("expires_in", &self.expires_in).field("refresh_token", &"[Redacted]").finish()
    }
}
pub struct Credentials {
  access_token: String,
  scope: String,
  expires_in: u64,
  refresh_token: Option<String>,
  user_id: Option<u64>,

  received_at: u64,
}
impl Credentials {
  pub fn new(access_token: String, scope: String, expires_in: u64, refresh_token: Option<String>, user_id: Option<u64>) -> Self { Self {
    access_token,
    user_id,
    scope,
    expires_in,
    refresh_token,
    received_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
  }}
  pub fn expires_at(&self) -> u64 {
    self.received_at + self.expires_in
  }
}
impl Debug for Credentials {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Credentials").field("access_token", &"[Redacted]").field("scope", &self.scope).field("expires_in", &self.expires_in).field("refresh_token", &"[Redacted]").field("received_at", &self.received_at).finish()
  }
}
impl From<TokenResponse> for Credentials {
  fn from(response: TokenResponse) -> Self {
    Self::new(response.access_token, response.scope, response.expires_in, Some(response.refresh_token), Some(response.user_id))
  }
}

pub struct Auth {
  client_id: String,
  client_secret: String,
  /// Authorisation Configuration
  redirect_uri: Option<String>,
  /// Credentials for the current session
  credentials: Option<Credentials>,
  
}
impl Auth {
  pub fn new(client_id: String, client_secret: String, redirect_uri: Option<String>) -> Self { Self {
    client_id,
    client_secret,
    redirect_uri,
    credentials: None,
  }}
  
  pub fn client(&self) -> Result<Client> {
    let mut headers = HeaderMap::new();
    let header_auth = format!("Basic {}", BASE64_STANDARD.encode(format!("{}:{}", self.client_id, self.client_secret))); 
    headers.insert("Authorization", header_auth.parse()?); // TODO: use better error 

    Client::builder().default_headers(headers).build().map_err(Into::into)
  } 

  pub fn refresh_creds(&mut self) -> Result<()> {
    let client = self.client()?;
    let creds = self.credentials.as_mut().ok_or(AuthError::Unauthenticated)?;
    
    if creds.refresh_token.is_some() {
      let mut params = HashMap::new();
      params.insert("grant_type", "refresh_token");
      params.insert("refresh_token", creds.refresh_token.as_ref().unwrap());

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
