use std::{fmt::Debug, ops::Deref};

use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

use crate::{
  api::{AuthUser, GrantType, TokenResponse},
  error::AuthError,
  Result,
};

/// A simple struct for storing client credentials, with a custom Debug impl that redacts the client secret.
#[derive(Clone, Serialize, Deserialize)]
pub struct ClientCreds {
  client_id: String,
  client_secret: String,
}
impl ClientCreds {
  /// Create a new `ClientCreds` struct with the given client_id and client_secret
  pub fn new(client_id: String, client_secret: String) -> Self {
    Self { client_id, client_secret }
  }
  /// Returns the client_id
  pub fn id(&self) -> &str {
    &self.client_id
  }
  /// Returns the client_secret
  pub fn secret(&self) -> &str {
    &self.client_secret
  }
  /// Returns the client creds as a tuple of (id, secret)
  pub fn as_tuple(&self) -> (&str, &str) {
    (&self.client_id, &self.client_secret)
  }
}
impl std::fmt::Debug for ClientCreds {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ClientCreds")
      .field("client_id", &self.client_id)
      .field("client_secret", &"[REDACTED]")
      .finish()
  }
}

/// A struct for storing authenticated credentials, including the grant type, experation, and associated user information.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCreds {
  client_credentials: ClientCreds,
  grant_type: GrantType,

  access_token: Token,
  scope: String,
  expires_in: i64,
  refresh_token: Option<Token>,
  user_id: Option<u64>,
  user: Option<AuthUser>,

  received_at: i64,
}
impl AuthCreds {
  /// Create new `AuthCreds` from a `TokenResponse`
  pub fn new(grant_type: GrantType, client_credentials: ClientCreds, response: TokenResponse) -> Self {
    let TokenResponse {
      access_token,
      user_id,
      user,
      scope,
      expires_in,
      refresh_token,
    } = response;

    Self {
      grant_type,
      client_credentials,
      access_token: access_token.into(),
      user_id,
      user,
      scope,
      expires_in,
      refresh_token: refresh_token.map(Token::from),
      received_at: chrono::Utc::now().timestamp(),
    }
  }
  /// Get the country code of the user, if available
  pub fn country_code(&self) -> Option<&CountryCode> {
    Some(&self.user.as_ref()?.country_code)
  }
  /// Get the unix timestamp of when the token expires
  pub fn expires_at(&self) -> i64 {
    self.received_at + self.expires_in
  }
  /// Check if the token is expired
  pub fn is_expired(&self) -> bool {
    self.expires_at() <= chrono::Utc::now().timestamp()
  }
  /// Return the current access token
  pub fn access_token(&self) -> &str {
    &self.access_token
  }
  /// Return the current refresh token, if available
  pub fn refresh_token(&self) -> Result<&str> {
    self.refresh_token.as_deref().ok_or(AuthError::MissingRefreshToken.into())
  }
  /// Returns the user id associated with the credentials, if available
  pub fn user_id(&self) -> Option<&u64> {
    self.user_id.as_ref()
  }
  /// Returns the `AuthUser` associated with the credentials, if available
  pub fn auth_user(&self) -> Option<&AuthUser> {
    self.user.as_ref()
  }
  /// Returns the grant type of the credentials
  pub fn grant_type(&self) -> &GrantType {
    &self.grant_type
  }
  /// Returns the client credentials
  pub fn client_credentials(&self) -> &ClientCreds {
    &self.client_credentials
  }
  /// Returns the scope of the credentials
  pub fn scope(&self) -> &str {
    &self.scope
  }
}

/// Wrapper around `String` for storing access tokens with a custom Debug impl that redacts the token.
#[derive(Serialize, Deserialize)]
pub struct Token(String);
impl From<String> for Token {
  fn from(token: String) -> Self {
    Self(token)
  }
}
impl Debug for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Token").field(&"[REDACTED]").finish()
  }
}
impl Deref for Token {
  type Target = str;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
