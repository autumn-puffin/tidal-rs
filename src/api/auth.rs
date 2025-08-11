use isocountry::CountryCode;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
  pub access_token: String,
  pub scope: String,
  pub expires_in: i64,
  #[serde(default)]
  pub user_id: Option<u64>,
  pub user: Option<AuthUser>,
  #[serde(default)]
  pub refresh_token: Option<String>,
}
impl Debug for TokenResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("TokenResponse")
      .field("access_token", &"[Redacted]")
      .field("user_id", &self.user_id)
      .field("user", &self.user)
      .field("scope", &self.scope)
      .field("expires_in", &self.expires_in)
      .field("refresh_token", &"[Redacted]")
      .finish()
  }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthUser {
  pub user_id: u64,
  pub email: String,
  pub country_code: CountryCode,
  pub full_name: Option<String>,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub nickname: Option<String>,
  pub username: String,
  pub address: Option<String>,
  pub city: Option<String>,
  pub postal_code: Option<String>,
  pub us_state: Option<String>,
  pub phone_number: Option<String>,
  pub birthday: Option<u64>,
  pub channel_id: u64,
  pub parent_id: u64,
  #[serde(rename = "acceptedEULA")]
  pub accepted_eula: bool,
  pub created: u64,
  pub updated: u64,
  pub facebook_uid: u64,
  pub apple_uid: Option<u64>,
  pub google_uid: Option<u64>,
  pub account_link_created: bool,
  pub email_verified: bool,
  pub new_user: bool,
}
