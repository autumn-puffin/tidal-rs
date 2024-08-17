use std::collections::HashMap;
use crate::{error::ApiErrorResponse, Error, Result};
use super::{credentials::Token, oauth, AuthError, Credentials, TokenResponse};
use serde::Deserialize;


pub trait ClientFlow {
  fn client_login(&mut self) -> Result<()>;
}
impl ClientFlow for super::Auth {
  fn client_login(&mut self) -> Result<()> {
    let client = self.client()?;

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");

    let res = client
      .post("https://auth.tidal.com/v1/oauth2/token")
      .form(&params).send()?;

    self.credentials = Some(res.json::<TokenResponse>()?.into());
    Ok(())
  } 
}

pub trait UserFlow {
  fn user_login_init(&self) -> Result<UserFlowInfo>;
  fn user_login_finalize(&mut self, code: String, info: UserFlowInfo) -> Result<()>;
}
impl UserFlow for super::Auth {
  fn user_login_init(&self) -> Result<UserFlowInfo> {
    let redirect_uri = self.redirect_uri.clone().ok_or(AuthError::MissingRedirectUri)?;
    let scopes = ["user.read".to_string()]; // TODO: make this configurable
    
    let (pkce_challenge, pkce_verifier) = oauth::pkce::new_random_sha256();
    let auth_url = format!(
      "https://auth.tidal.com/v1/oauth2/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}&code_challenge_method=S256&code_challenge={}", 
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
    let redirect_uri = self.redirect_uri.clone().ok_or(AuthError::MissingRedirectUri)?;

    let client = self.client()?;
    let verifier = info.pkce_verifier.as_string();
    
    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("client_id", self.client_credentials.id());
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

pub trait DeviceFlow {
  fn device_login_init(&self) -> Result<DeviceFlowResponse>;
  fn try_device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()>;
  fn device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()>;
}
impl DeviceFlow for super::Auth {
  fn device_login_init(&self) -> Result<DeviceFlowResponse> {
    let client = self.client()?;

    let mut params = HashMap::new();
    params.insert("scope", "r_usr+w_usr+w_sub");
    params.insert("client_id", &self.client_credentials.id());

    let res = client
      .post("https://auth.tidal.com/v1/oauth2/device_authorization")
      .form(&params).send()?;

    Ok(res.json()?)
  }
  fn try_device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()> {
    let client = self.client()?;

    let mut params = HashMap::new();
    params.insert("scope", "r_usr+w_usr+w_sub");
    params.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
    params.insert("client_id", &self.client_credentials.id());
    params.insert("device_code", &response.device_code);
    
    
    let res = client
      .post("https://auth.tidal.com/v1/oauth2/token")
      .form(&[
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ("device_code", &response.device_code),
        ("client_id", &self.client_credentials.id()),
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

#[derive(Debug)]
pub struct UserFlowInfo {
  pub auth_url: String,
  pub pkce_verifier: oauth::pkce::PkceVerifier,
}

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