use serde::Deserialize;

use crate::auth::AuthError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  AuthError(AuthError),
}

impl<T> From<T> for Error
where
  T: Into<AuthError>,
{
  fn from(err: T) -> Self {
    Error::AuthError(err.into())
  }
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
  pub status: u16,
  pub error: String,
  pub sub_status: Option<u16>,
  pub error_description: String,
}