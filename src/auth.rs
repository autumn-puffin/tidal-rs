use std::{collections::HashMap, fmt::Debug};
use crate::{client::ClientCreds, endpoints::Endpoint, error::ApiErrorResponse, utils::{client_login_impl, oauth_request_helper, post_request_helper}, Error, Result};
use credentials::GrantType;
use flows::{DeviceFlowResponse, UserFlowInfo};
use reqwest::blocking::Client;
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
impl ClientFlow for AuthClient {
  fn client_login(&mut self) -> Result<()> {
    self.credentials = Some(client_login_impl(&self.client_credentials)?);
    Ok(())
  } 
}
impl UserFlow for AuthClient {
  fn user_login_init(&self) -> Result<UserFlowInfo> {
    let redirect_uri = self.redirect_uri.as_deref().ok_or(AuthError::MissingRedirectUri)?;
    let scopes = ["user.read".to_string()]; // TODO: make this configurable
    
    let (pkce_challenge, pkce_verifier) = oauth::pkce::new_random_sha256();
    let auth_url = format!(
      "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&code_challenge_method=S256&code_challenge={}", 
      Endpoint::LoginAuthorize.to_string(),
      self.client_credentials.id(), 
      &redirect_uri, 
      scopes.join("+"), 
      pkce_challenge.as_string()
    );


    Ok(UserFlowInfo { 
      auth_url,
      pkce_verifier,
    })
  }
  fn user_login_finalize(&mut self, code: String, info: UserFlowInfo) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::AuthorizationCode;
    let client_credentials = &self.client_credentials;
    
    let redirect_uri = self.redirect_uri.as_deref().ok_or(AuthError::MissingRedirectUri)?;
    let verifier = info.pkce_verifier.as_string();
    
    let mut params = HashMap::new();
    params.insert("client_id", self.client_credentials.id());
    params.insert("code", &code);
    params.insert("redirect_uri", redirect_uri);
    params.insert("code_verifier", &verifier);

    let res = oauth_request_helper(endpoint, grant, &client_credentials, Some(params)).send()?;

    let credentials = Credentials::new(grant, client_credentials.clone(), res.json::<TokenResponse>()?);
    self.credentials = Some(credentials);
    Ok(())

  }
}
impl DeviceFlow for AuthClient {
  fn device_login_init(&self) -> Result<DeviceFlowResponse> {
    let client = Client::new();
    let endpoint = Endpoint::OAuth2DeviceAuth;
    let client_credentials = &self.client_credentials;


    let mut params = HashMap::new();
    params.insert("scope", "r_usr+w_usr+w_sub");
    params.insert("client_id", &self.client_credentials.id());

    let res = post_request_helper(&client, endpoint, &client_credentials)
      .form(&params).send()?;

    Ok(res.json()?)
  }
  fn try_device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::DeviceCode;
    let client_credentials = &self.client_credentials;

    let mut params = HashMap::new();
    params.insert("scope", "r_usr+w_usr+w_sub");
    params.insert("client_id", &self.client_credentials.id());
    params.insert("device_code", &response.device_code);
    
    let res = oauth_request_helper(endpoint, grant, &client_credentials, Some(params)).send()?;
    

    if res.status().is_success() {
      let credentials = Credentials::new(GrantType::DeviceCode, client_credentials.clone(), res.json::<TokenResponse>()?);
      self.credentials = Some(credentials);
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
  fn device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()> {
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
impl RefreshFlow for AuthClient {
  fn refresh(&mut self) -> Result<()> {
    self.credentials.as_mut().ok_or(AuthError::Unauthenticated)?.refresh()
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
