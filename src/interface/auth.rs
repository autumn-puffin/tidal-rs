//! The authentication interface for the Tidal API
//!
//! This module contains the traits and structs for handling authentication with the Tidal API,
//! allowing the user to implement as many or as few of the flows as they want

pub mod flows;
pub use flows::*;
use serde::{Deserialize, Serialize};

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
