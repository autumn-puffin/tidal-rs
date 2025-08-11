use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct UserFlowInfo {
  auth_url: String,
  pkce_verifier: String,
}
impl UserFlowInfo {
  pub fn new(auth_url: String, pkce_verifier: String) -> Self {
    Self { auth_url, pkce_verifier }
  }
  pub fn url(&self) -> &str {
    &self.auth_url
  }
  pub fn verifier(&self) -> &str {
    &self.pkce_verifier
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceFlowResponse {
  pub device_code: String,
  pub user_code: String,
  pub verification_uri: String,
  pub verification_uri_complete: String,
  pub expires_in: i64,
  pub interval: i64,
}

/// The type of grant being used for authentication
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GrantType {
  ClientCredentials,
  AuthorizationCode,
  DeviceCode,
  RefreshToken,
}
impl GrantType {
  pub fn as_str(&self) -> &str {
    match self {
      Self::ClientCredentials => "client_credentials",
      Self::AuthorizationCode => "authorization_code",
      Self::DeviceCode => "urn:ietf:params:oauth:grant-type:device_code",
      Self::RefreshToken => "refresh_token",
    }
  }
}
