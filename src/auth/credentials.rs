use crate::client::ClientCreds;

use super::TokenResponse;
use std::{fmt::Debug, ops::Deref, rc::Rc};

#[derive(Debug)]
pub struct Credentials {
  client_credentials: Rc<ClientCreds>,

  access_token: Token,
  scope: String,
  expires_in: u64,
  refresh_token: Option<Token>,
  user_id: Option<u64>,

  received_at: u64,
}
impl Credentials {
  pub fn new(client_credentials: &Rc<ClientCreds>, response: TokenResponse) -> Self {
    let TokenResponse { access_token, user_id, scope, expires_in, refresh_token } = response;
    
    Self {
      client_credentials: client_credentials.clone(),
      access_token: access_token.into(),
      user_id,
      scope,
      expires_in,
      refresh_token: refresh_token.map(Token::from),
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
