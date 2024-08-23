use crate::{auth::AuthError, users::UsersError};
use serde::Deserialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  AuthError(AuthError),
  UsersError(UsersError),

  ReqwestError(reqwest::Error),
  ApiError(ApiErrorResponse),
  UrlParseError(url::ParseError),
}
impl From<AuthError> for Error {
  fn from(err: AuthError) -> Self {
    Error::AuthError(err)
  }
}
impl From<UsersError> for Error {
  fn from(err: UsersError) -> Self {
    Error::UsersError(err)
  }
}
impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
    Error::ReqwestError(err)
  }
}
impl From<ApiErrorResponse> for Error {
  fn from(err: ApiErrorResponse) -> Self {
    match err.error.as_str() {
      "authorization_pending" => Error::AuthError(AuthError::AuthorizationPending),
      _ => Error::ApiError(err),
    }
  }
}
impl From<url::ParseError> for Error {
  fn from(err: url::ParseError) -> Self {
    Error::UrlParseError(err)
  }
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
  pub status: u16,
  pub error: String,
  pub sub_status: Option<u16>,
  pub error_description: String,
}
