use crate::{Error, Result};
use serde::Deserialize;

/// Authenticate using only client credentials
///
/// Client flow uses only the client credentials to authenticate, offering only basic access
#[deprecated]
pub trait ClientFlow {
  /// Establish authentication using only the client credentials
  fn client_login(&mut self) -> Result<()>;
}
/// Authenticate using user credentials via a redirect
///
/// User flow works in two stages, first `user_login_init()` is called, which should return a
/// properly structured URL and a PKCE verifier. The user should then visit the URL and authorize
/// the application. Once the user has authorized the application, they will be redirected to a
/// specified redirect URI with a code as a query parameter. This code, along with the PKCE
/// verifier should be passed to `user_login_finalize()`, which will complete the login process.
#[deprecated]
pub trait UserFlow {
  /// Returns a URL and PKCE verifier for the user to authorize the application
  fn user_login_init(&self) -> Result<UserFlowInfo>;
  /// Completes the user login process using the code and PKCE verifier
  fn user_login_finalize(&mut self, code: String, info: UserFlowInfo) -> Result<()>;
}
/// Authenticate using user credentials via a device code
///
/// Device flow is similar to user flow, but is designed for devices with limited input
/// capabilities. The device flow is initiated with `device_login_init()`, which should return a
/// user code and a verification URI to be displayed to the user, as well as a device code, expiry
/// time, and polling interval, which will be used when finalizing authentication. The user should
/// visit the verification URI, enter the user code, and authenticate the application. Once the
/// user has authenticated the application, `try_device_login_finalize()` can be called to complete
/// the login process. Since device flow does not require direct user input, it is possible, and
/// recomended to call `device_login_finalize()` instead, which will automatically poll the server
/// until either the user has authenticated the application, or the flow has expired.
#[deprecated]
pub trait DeviceFlow {
  /// Returns a user code, verification URI, device code, expiry time, and polling interval
  fn device_login_init(&self) -> Result<DeviceFlowResponse>;
  /// Completes the device login process
  fn try_device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()>;
  /// Polls the server until the user has authenticated, or the flow has expired
  fn device_login_finalize(&mut self, response: &DeviceFlowResponse) -> Result<()> {
    let interval = response.interval;
    let max_retries = response.expires_in / interval;

    let mut i: i64 = 0;
    while i < max_retries {
      match self.try_device_login_finalize(response) {
        Err(Error::AuthError(crate::error::AuthError::AuthorizationPending)) => {
          i += 1;
          std::thread::sleep(std::time::Duration::from_secs(interval as u64));
        }
        res => return res,
      }
    }
    Err(crate::error::AuthError::MaxRetriesReached)?
  }
}

/// Refresh credentials when they expire
///
/// Refresh flow is used to refresh credentials when they expire
#[deprecated]
pub trait RefreshFlow {
  /// Refresh credentials
  fn refresh(&mut self) -> Result<()>;
}

#[derive(Debug)]
pub struct UserFlowInfo {
  auth_url: String,
  pkce_verifier: String,
}
impl UserFlowInfo {
  pub fn new(auth_url: String, pkce_verifier: String) -> Self {
    Self { auth_url, pkce_verifier }
  }
  pub fn url(&self) -> &str {
    &self.auth_url
  }
  pub fn verifier(&self) -> &str {
    &self.pkce_verifier
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceFlowResponse {
  pub device_code: String,
  pub user_code: String,
  pub verification_uri: String,
  pub verification_uri_complete: String,
  pub expires_in: i64,
  pub interval: i64,
}
