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
impl Error {
  pub fn is_unauthenticated(&self) -> bool {
    matches!(self, Self::AuthError(AuthError::Unauthenticated))
  }
}
impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
    Error::ReqwestError(err)
  }
}
impl From<url::ParseError> for Error {
  fn from(err: url::ParseError) -> Self {
    Error::UrlParseError(err)
  }
}

#[derive(Debug)]
pub enum AuthError {
  AuthorizationPending,
  MissingRedirectUri,
  MaxRetriesReached,
  Unauthenticated,
  MissingRefreshToken,
}
impl From<AuthError> for Error {
  fn from(err: AuthError) -> Self {
    Error::AuthError(err)
  }
}

#[derive(Debug)]
pub enum UsersError {
  NoCurrentUser,
}
impl From<UsersError> for Error {
  fn from(err: UsersError) -> Self {
    Error::UsersError(err)
  }
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
  #[serde(alias = "httpStatus")]
  pub status: u16,
  pub error: Option<String>,
  #[serde(alias = "subStatus")]
  pub sub_status: Option<u16>,
  #[serde(alias = "description")]
  pub error_description: Option<String>,
}
impl From<ApiErrorResponse> for Error {
  fn from(err: ApiErrorResponse) -> Self {
    match err.error.as_deref() {
      Some("authorization_pending") => Error::AuthError(AuthError::AuthorizationPending),
      _ => Error::ApiError(err),
    }
  }
}
