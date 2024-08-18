use std::collections::HashMap;
use crate::{endpoints::Endpoint, error::ApiErrorResponse, utils::{oauth_request_helper, post_request_helper}, Error, Result};
use super::{credentials::GrantType, oauth, AuthError, Credentials, TokenResponse};
use reqwest::blocking::Client;
use serde::Deserialize;


pub trait ClientFlow {
  fn client_login(&mut self) -> Result<()>;
}
impl ClientFlow for super::Auth {
  fn client_login(&mut self) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::ClientCredentials;
    let client_credentials = &self.client_credentials;

    let res = oauth_request_helper( endpoint, grant, client_credentials, None).send()?;

    self.credentials = Some(Credentials::new(grant, client_credentials, res.json::<TokenResponse>()?));
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
    
    let redirect_uri = self.redirect_uri.as_ref().ok_or(AuthError::MissingRedirectUri)?;
    let verifier = info.pkce_verifier.as_string();
    
    let mut params = HashMap::new();
    params.insert("client_id", self.client_credentials.id());
    params.insert("code", &code);
    params.insert("redirect_uri", redirect_uri);
    params.insert("code_verifier", &verifier);

    let res = oauth_request_helper(endpoint, grant, &client_credentials, Some(params)).send()?;

    self.credentials = Some(Credentials::new(grant, client_credentials, res.json::<TokenResponse>()?));
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
      self.credentials = Some(Credentials::new(GrantType::DeviceCode, client_credentials, res.json::<TokenResponse>()?));
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

pub trait RefreshFlow {
  fn refresh_creds(&mut self) -> Result<()>;
}
impl RefreshFlow for super::Auth {
  fn refresh_creds(&mut self) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::RefreshToken;
    let client_credentials = &self.client_credentials;
    let creds = self.credentials.as_mut().ok_or(AuthError::Unauthenticated)?;

    if let Some(refresh_token) = creds.refresh_token() {
      let mut params = HashMap::new();
      params.insert("refresh_token", refresh_token);

      let res = oauth_request_helper(endpoint, grant, client_credentials, Some(params)).send()?;

      if res.status().is_success() {
        self.credentials = Some(Credentials::new(GrantType::RefreshToken, client_credentials, res.json::<TokenResponse>()?));
      } else {
        let err = res.json::<ApiErrorResponse>()?;
        Err(AuthError::ApiError(err))?
      }
      Ok(())
    } else {
      self.client_login()
    }
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