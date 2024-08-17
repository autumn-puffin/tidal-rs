use super::TokenResponse;
use std::{fmt::Debug, ops::Deref};

pub struct Credentials {
  access_token: Token,
  scope: String,
  expires_in: u64,
  refresh_token: Option<Token>,
  user_id: Option<u64>,

  received_at: u64,
}
impl Credentials {
  pub fn new(access_token: Token, scope: String, expires_in: u64, refresh_token: Option<Token>, user_id: Option<u64>) -> Self {
    Self {
      access_token,
      user_id,
      scope,
      expires_in,
      refresh_token,
      received_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
    }
  }
  pub fn expires_at(&self) -> u64 {
    self.received_at + self.expires_in
  }
  pub fn access_token(&self) -> &str {
    &self.access_token
  }
  pub fn refresh_token(&self) -> Option<&str> {
    self.refresh_token.as_deref()
  }
  pub fn user_id(&self) -> Option<&u64> {
    self.user_id.as_ref()
  }
}
impl Debug for Credentials {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Credentials")
      .field("access_token", &"[Redacted]")
      .field("scope", &self.scope)
      .field("expires_in", &self.expires_in)
      .field("refresh_token", &"[Redacted]")
      .field("received_at", &self.received_at)
      .finish()
  }
}
impl From<TokenResponse> for Credentials {
  fn from(response: TokenResponse) -> Self {
    Self::new(
      response.access_token.into(),
      response.scope,
      response.expires_in,
      response.refresh_token.map(Token::from),
      response.user_id,
    )
  }
}

pub struct Token(pub String);
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
impl Token {
  pub fn new(token: String) -> Self {
    Self(token)
  }
  pub fn as_str(&self) -> &str {
    &self.0
  }
}
