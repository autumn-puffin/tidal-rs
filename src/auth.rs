use std::fmt::Debug;
use crate::{error::ApiErrorResponse, Result};
use base64::prelude::*;
use reqwest::{blocking::Client, header::HeaderMap};
use serde::Deserialize;

mod oauth;

mod client_flow {}
mod user_flow {
  use std::collections::HashMap;
  use crate::Result;
  use super::{oauth, AuthError, Token, TokenResponse};

  #[derive(Debug)]
  pub struct UserFlowInfo {
    pub auth_url: String,
    pub pkce_verifier: oauth::pkce::PkceVerifier,
  }
  
  impl super::Auth {
    pub fn user_login_init(&self) -> Result<UserFlowInfo> {
      let redirect_uri = self.redirect_uri.clone().ok_or(AuthError::MissingRedirectUri)?;
      let scopes = ["user.read".to_string()];
      
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
    
    pub fn user_login_finalize(&self, code: String, info: UserFlowInfo) -> Result<Token> {
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

      Ok(res.json::<TokenResponse>()?.into())

    }
  }
}
mod device_flow {
  use std::collections::HashMap;
  use crate::{error::ApiErrorResponse, Error, Result};
  use super::{AuthError, Token, TokenResponse};
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
    pub fn try_device_login_finalize(&self, response: &DeviceFlowResponse) -> Result<Token> {
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
        Ok(res.json::<TokenResponse>()?.into())
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
    pub fn device_login_finalize(&self, response: &DeviceFlowResponse) -> Result<Token> {
      let interval = response.interval;
      let max_retries = response.expires_in / interval;

      let mut i: u64 = 0;
      while i < max_retries {
        let res = self.try_device_login_finalize(response);
        match res {
          Ok(token) => return Ok(token),
          Err(Error::AuthError(AuthError::AuthorizationPending)) => i += 1,
          Err(e) => return Err(e),
        }

        std::thread::sleep(std::time::Duration::from_secs(interval));
      }
      Err(AuthError::MaxRetriesReached)?
    }
  }
}


#[derive(Deserialize)]
pub struct TokenResponse {
  access_token: String,
  token_type: String,
  scope: String,
  expires_in: u64,
  refresh_token: String,
}
impl Debug for TokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenResponse").field("access_token", &"[Redacted]").field("token_type", &self.token_type).field("scope", &self.scope).field("expires_in", &self.expires_in).field("refresh_token", &"[Redacted]").finish()
    }
}
pub struct Token {
  access_token: String,
  token_type: String,
  scope: String,
  expires_in: u64,
  refresh_token: String,

  auto_renew: bool,
  received_at: u64,
}
impl Token {
  pub fn new(access_token: String, token_type: String, scope: String, expires_in: u64, refresh_token: String) -> Self { Self {
    access_token,
    token_type,
    scope,
    expires_in,
    refresh_token,
    auto_renew: true,
    received_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
  }}
  pub fn expires_at(&self) -> u64 {
    self.received_at + self.expires_in
  }
}
impl Debug for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Token").field("access_token", &"[Redacted]").field("token_type", &self.token_type).field("scope", &self.scope).field("expires_in", &self.expires_in).field("refresh_token", &"[Redacted]").field("auto_renew", &self.auto_renew).field("received_at", &self.received_at).finish()
  }
}
impl From<TokenResponse> for Token {
  fn from(response: TokenResponse) -> Self {
    Self::new(response.access_token, response.token_type, response.scope, response.expires_in, response.refresh_token)
  }
}

pub struct Auth {
  client_id: String,
  client_secret: String,
  redirect_uri: Option<String>,
}
impl Auth {
  pub fn new(client_id: String, client_secret: String, redirect_uri: Option<String>) -> Self { Self {
    client_id,
    client_secret,
    redirect_uri,
  }}
  
  pub fn client(&self) -> Result<Client> {
    let mut headers = HeaderMap::new();
    let header_auth = format!("Basic {}", BASE64_STANDARD.encode(format!("{}:{}", self.client_id, self.client_secret))); 
    headers.insert("Authorization", header_auth.parse()?); // TODO: use better error 

    Client::builder().default_headers(headers).build().map_err(Into::into)
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
