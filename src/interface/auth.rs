//! The authentication interface for the Tidal API
//!
//! This module contains the traits and structs for handling authentication with the Tidal API,
//! allowing the user to implement as many or as few of the flows as they want

use crate::{api::Session, Result};
use chrono::Utc;

pub mod flows;
pub use flows::*;
use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

/// Access and manage authenticated credentials
///
/// Defines a client capable of being authenticated, required by other interfaces
pub trait Auth {
  /// A type which acts as the credentials for the client
  type Credentials: Credentials;

  /// Get a reference to the credentials
  fn get_credentials(&self) -> Result<&Self::Credentials>;
  /// Get a mutable reference to the credentials
  fn get_credentials_mut(&mut self) -> Result<&mut Self::Credentials>;

  /// Get the credentials, refreshing them if they are expired
  fn get_credentials_refresh(&mut self) -> Result<&Self::Credentials> {
    self.credentials_refresh()?;
    self.get_credentials()
  }
  /// Get the credentials, forcing a refresh
  fn get_credentials_force_refresh(&mut self) -> Result<&Self::Credentials> {
    self.credentials_force_refresh()?;
    self.get_credentials()
  }
  /// Refresh the credentials if they are expired
  fn credentials_refresh(&mut self) -> Result<()> {
    let credentials = self.get_credentials_mut()?;
    let expire_time = credentials.expires_at();

    let cur_time = Utc::now().timestamp();
    if expire_time <= cur_time {
      credentials.refresh()?;
    }
    Ok(())
  }
  /// Force a refresh of the credentials
  fn credentials_force_refresh(&mut self) -> Result<()> {
    self.get_credentials_mut()?.refresh()
  }
}

/// Basic requirements for authenticated credentials
///
/// Defines a type which can act as credentials for an authenticated client, requires an
/// implementation of the `RefreshFlow` trait
pub trait Credentials: RefreshFlow {
  /// Retuns the time at which the credentials expire, formatted as a unix timestamp
  fn expires_at(&self) -> i64;
  /// Returns the country code of the user associated with the credentials, if any
  fn country_code(&self) -> Option<&CountryCode>;
  /// Returns the id of the user associated with the credentials, if any
  fn user_id(&self) -> Option<&u64>;
}

pub trait Sessions: Auth {
  /// Get a session from the current authentication credentials
  fn get_session_from_auth(&self) -> Result<Session>;
  /// Get a session from it's specified id
  fn get_session(&self, session_id: &str) -> Result<Session>;
}

/// The type of grant being used for authentication
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug)]
pub enum AuthError {
  AuthorizationPending,
  MissingRedirectUri,
  MaxRetriesReached,
  Unauthenticated,
}
