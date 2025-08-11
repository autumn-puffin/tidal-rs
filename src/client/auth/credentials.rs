use chrono::Utc;
use isocountry::CountryCode;
use reqwest::blocking::Client as ReqwestClient;
use serde::{Deserialize, Serialize};

use super::{AuthUser, TokenResponse};
use crate::{
  client::ClientCreds,
  endpoints::Endpoint,
  error::{ApiErrorResponse, AuthError},
  interface::auth::*,
  utils::oauth_request_helper,
  Result,
};
use std::{fmt::Debug, ops::Deref};

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
      received_at: Utc::now().timestamp(),
    }
  }
  pub fn country_code(&self) -> Option<&CountryCode> {
    Some(&self.user.as_ref()?.country_code)
  }
  pub fn expires_at(&self) -> i64 {
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
  pub fn auth_user(&self) -> Option<&AuthUser> {
    self.user.as_ref()
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

  pub fn client_login_with_http_client(&mut self, http_client: &ReqwestClient) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::ClientCredentials;

    let res = oauth_request_helper(http_client, endpoint, grant, self.client_credentials(), None).send()?;

    *self = AuthCreds::new(grant, self.client_credentials().clone(), res.json::<TokenResponse>()?);
    Ok(())
  }
  pub fn refresh_with_http_client(&mut self, client: &ReqwestClient) -> Result<()> {
    if self.grant_type == GrantType::ClientCredentials {
      return self.client_login_with_http_client(client);
    }
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::RefreshToken;
    let client_credentials = self.client_credentials();
    let refresh_token = self.refresh_token().ok_or(AuthError::MissingRefreshToken)?;

    let params = &[("refresh_token", refresh_token)];

    let res = oauth_request_helper(client, endpoint, grant, client_credentials, Some(params)).send()?;
    if res.status().is_success() {
      *self = AuthCreds::new(GrantType::RefreshToken, client_credentials.clone(), res.json::<TokenResponse>()?);
    } else {
      return Err(res.json::<ApiErrorResponse>()?.into());
    }
    Ok(())
  }
}

#[derive(Serialize, Deserialize)]
pub struct Token(pub String);
impl Token {
  pub fn new(token: String) -> Self {
    Self(token)
  }
  pub fn as_str(&self) -> &str {
    &self.0
  }
}
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
