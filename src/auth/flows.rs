use crate::{Error, Result};
use super::oauth;

use serde::Deserialize;


pub trait ClientFlow {
  fn client_login(&mut self) -> Result<()>;
}

pub trait UserFlow {
  fn user_login_init(&self) -> Result<UserFlowInfo>;
  fn user_login_finalize(&mut self, code: String, info: UserFlowInfo) -> Result<()>;
}

pub trait DeviceFlow {
  fn device_login_init(&self) -> Result<DeviceFlowResponse>;
  fn try_device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()>;
  fn device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()> {
    let interval = response.interval;
    let max_retries = response.expires_in / interval;

    let mut i: u64 = 0;
    while i < max_retries { match self.try_device_login_finalize(response) {
      Err(Error::AuthError(super::AuthError::AuthorizationPending)) => {
          i += 1;
        std::thread::sleep(std::time::Duration::from_secs(interval));
      },
      res => return res,
    }}
    Err(super::AuthError::MaxRetriesReached)?
  }}

pub trait RefreshFlow {
  fn refresh(&mut self) -> Result<()>;
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