use crate::Result;
use chrono::Utc;

pub mod flows;
pub use flows::*;
use isocountry::CountryCode;

pub trait Auth {
  type Credentials: Credentials;

  fn get_credentials(&self) -> Result<&Self::Credentials>;
  fn get_credentials_mut(&mut self) -> Result<&mut Self::Credentials>;

  fn get_credentials_refresh(&mut self) -> Result<&Self::Credentials> {
    self.credentials_refresh()?;
    self.get_credentials()
  }
  fn get_credentials_force_refresh(&mut self) -> Result<&Self::Credentials> {
    self.credentials_force_refresh()?;
    self.get_credentials()
  }
  fn credentials_refresh(&mut self) -> Result<()> {
    let credentials = self.get_credentials_mut()?;
    let expire_time = credentials.expires_at();

    let cur_time = Utc::now().timestamp() as u64;
    if expire_time <= cur_time {
      credentials.refresh()?;
    }
    Ok(())
  }
  fn credentials_force_refresh(&mut self) -> Result<()> {
    self.get_credentials_mut()?.refresh()
  }
}

pub trait Credentials: RefreshFlow {
  fn expires_at(&self) -> u64;
  fn country_code(&self) -> Option<&CountryCode>;
  fn user_id(&self) -> Option<&u64>;
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

#[derive(Debug)]
pub enum AuthError {
  AuthorizationPending,
  MissingRedirectUri,
  MaxRetriesReached,
  Unauthenticated,
}
