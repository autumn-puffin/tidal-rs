use crate::{client::ClientCreds, endpoints::Endpoint, error::ApiErrorResponse, utils::{client_login_impl, oauth_request_helper}, Result};
use super::{ClientFlow, RefreshFlow, TokenResponse};
use std::{collections::HashMap, fmt::Debug, ops::Deref};

#[derive(Debug)]
pub struct Credentials {
  client_credentials: ClientCreds,
  grant_type: GrantType,

  access_token: Token,
  scope: String,
  expires_in: u64,
  refresh_token: Option<Token>,
  user_id: Option<u64>,

  received_at: u64,
}
impl Credentials {
  pub fn new(grant_type: GrantType, client_credentials: ClientCreds, response: TokenResponse) -> Self {
    let TokenResponse { access_token, user_id, scope, expires_in, refresh_token } = response;
    
    Self {
      grant_type,
      client_credentials: client_credentials,
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
  pub fn grant_type(&self) -> &GrantType {
    &self.grant_type
  }
  pub fn client_credentials(&self) -> &ClientCreds {
    &self.client_credentials
  }
  pub fn scope(&self) -> &str {
    &self.scope
  }
}
impl ClientFlow for Credentials {
  fn client_login(&mut self) -> Result<()> {
    *self = client_login_impl(&self.client_credentials())?;
    Ok(())
  }
}
impl RefreshFlow for Credentials {
  fn refresh(&mut self) -> Result<()> {
    match self.grant_type() {
      GrantType::ClientCredentials => self.client_login(),
      _ => {
        let endpoint = Endpoint::OAuth2Token;
        let grant = GrantType::RefreshToken;
        let client_credentials = self.client_credentials();
        let refresh_token = self.refresh_token().unwrap();

        let mut params = HashMap::new();
        params.insert("refresh_token", refresh_token);

        let res = oauth_request_helper(endpoint, grant, client_credentials, Some(params)).send()?;
        if res.status().is_success() {
          *self = Credentials::new(GrantType::RefreshToken, client_credentials.clone(), res.json::<TokenResponse>()?);
        } else {
          return Err(res.json::<ApiErrorResponse>()?.into());
        }
        Ok(())
      },
    }
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

#[derive(Debug, Clone, Copy)]
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