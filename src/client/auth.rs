//! Authentication interface for the TIDAL API

pub use crate::interface::auth::{flows::*, *};
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod credentials;
pub use credentials::AuthCreds;

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
  access_token: String,
  scope: String,
  expires_in: i64,
  #[serde(default)]
  user_id: Option<u64>,
  user: Option<AuthUser>,
  #[serde(default)]
  refresh_token: Option<String>,
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
