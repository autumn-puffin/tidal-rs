use serde::Deserialize;

use crate::auth::AuthError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  AuthError(AuthError),
}

impl From<AuthError> for Error {
  fn from(err: AuthError) -> Self {
    Error::AuthError(err)
  }
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
  pub status: u16,
  pub error: String,
  pub sub_status: Option<u16>,
  pub error_description: String,
}