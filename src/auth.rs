use std::{collections::HashMap, fmt::Debug};
use crate::{error::ApiErrorResponse, Result};
use base64::prelude::*;
use reqwest::{blocking::Client, header::HeaderMap};
use serde::Deserialize;

mod oauth;

mod client_flow {
  use std::collections::HashMap;

  use serde::Deserialize;

  use crate::Result;
  use super::Credentials;

  #[derive(Deserialize)]
  struct LimitedTokenResponse {
    access_token: String,
    scope: String,
    expires_in: u64,
  }

  impl From<LimitedTokenResponse> for Credentials {
    fn from(response: LimitedTokenResponse) -> Self {
      Credentials::new(response.access_token, response.scope, response.expires_in, None, None)
    }
  }

  impl super::Auth {
    pub fn client_login(&mut self) -> Result<()> {
      let client = self.client()?;

      let mut params = HashMap::new();
      params.insert("grant_type", "client_credentials");

      let res = client
        .post("https://auth.tidal.com/v1/oauth2/token")
        .form(&params).send()?;

      self.credentials = Some(res.json::<LimitedTokenResponse>()?.into());
      Ok(())
    } 
  }
}
mod user_flow {
  use std::collections::HashMap;
  use crate::Result;
  use super::{oauth, AuthError, TokenResponse};

  #[derive(Debug)]
  pub struct UserFlowInfo {
    pub auth_url: String,
    pub pkce_verifier: oauth::pkce::PkceVerifier,
  }
  
  impl super::Auth {
    pub fn user_login_init(&self) -> Result<UserFlowInfo> {
      let redirect_uri = self.redirect_uri.clone().ok_or(AuthError::MissingRedirectUri)?;
      let scopes = ["user.read".to_string()]; // TODO: make this configurable
      
      let (pkce_challenge, pkce_verifier) = oauth::pkce::new_random_sha256();
      let auth_url = format!(
        "https://auth.tidal.com/v1/oauth2/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}&code_challenge_method=S256&code_challenge={}", 
        &self.client_id, 
        &redirect_uri, 
        scopes.join("+"), 
        pkce_challenge.as_string()
      );


      Ok(UserFlowInfo { 
        auth_url,
        pkce_verifier,
      })
    }
    
    pub fn user_login_finalize(&mut self, code: String, info: UserFlowInfo) -> Result<()> {
      let redirect_uri = self.redirect_uri.clone().ok_or(AuthError::MissingRedirectUri)?;

      let client = self.client()?;
      let verifier = info.pkce_verifier.as_string();
      
      let mut params = HashMap::new();
      params.insert("grant_type", "authorization_code");
      params.insert("client_id", &self.client_id);
      params.insert("code", &code);
      params.insert("redirect_uri", &redirect_uri);
      params.insert("code_verifier", &verifier);

      let res = client
        .post("https://auth.tidal.com/v1/oauth2/token")
        .form(&params).send()?;

      self.credentials = Some(res.json::<TokenResponse>()?.into());
      Ok(())

    }
  }
}
mod device_flow {
  use std::collections::HashMap;
  use crate::{error::ApiErrorResponse, Error, Result};
  use super::{AuthError, TokenResponse};
  use serde::Deserialize;

  #[derive(Debug, Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct DeviceFlowResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u64,
    pub interval: u64,
  }

  impl super::Auth {
    pub fn device_login_init(&self) -> Result<DeviceFlowResponse> {
      let client = self.client()?;

      let mut params = HashMap::new();
      params.insert("scope", "r_usr+w_usr+w_sub");
      params.insert("client_id", &self.client_id);

      let res = client
        .post("https://auth.tidal.com/v1/oauth2/device_authorization")
        .form(&params).send()?;

      Ok(res.json()?)
    }
    pub fn try_device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()> {
      let client = self.client()?;

      let mut params = HashMap::new();
      params.insert("scope", "r_usr+w_usr+w_sub");
      params.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
      params.insert("client_id", &self.client_id);
      params.insert("device_code", &response.device_code);
      
      
      let res = client
        .post("https://auth.tidal.com/v1/oauth2/token")
        .form(&[
          ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
          ("device_code", &response.device_code),
          ("client_id", &self.client_id),
        ]).send()?;
      

      if res.status().is_success() {
        self.credentials = Some(res.json::<TokenResponse>()?.into());
        Ok(())
      } else {
        let err = res.json::<ApiErrorResponse>()?;
        match err.error.as_str() {
          "authorization_pending" => {
            Err(AuthError::AuthorizationPending)?
          },
          _ => Err(AuthError::ApiError(err))?
        }
      }
    }
    pub fn device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()> {
      let interval = response.interval;
      let max_retries = response.expires_in / interval;

      let mut i: u64 = 0;
      while i < max_retries { match self.try_device_login_finalize(response) {
        Err(Error::AuthError(AuthError::AuthorizationPending)) => {
            i += 1;
          std::thread::sleep(std::time::Duration::from_secs(interval));
        },
        res => return res,
      }}
      Err(AuthError::MaxRetriesReached)?
    }
  }
}


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
