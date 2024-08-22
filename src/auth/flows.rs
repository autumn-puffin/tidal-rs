use crate::Result;
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
  fn device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()>;
}

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