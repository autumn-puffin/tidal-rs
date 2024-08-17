pub mod base_urls {
  pub use prod::*;
  pub mod prod {
    pub const API_URL_V1: &str = "https://api.tidal.com/v1/";
    pub const API_URL_V2: &str = "https://api.tidal.com/v2/";
    pub const OPEN_API_URL: &str = "https://openapi.tidal.com/v2/";
    pub const AUTH_URL: &str = "https://auth.tidal.com/v1/";
    pub const LOGIN_URL: &str = "https://login.tidal.com/";
  }
  pub mod stage {
    pub const API_URL_V1: &str = "https://api.stage.tidal.com/v1/";
    pub const API_URL_V2: &str = "https://api.stage.tidal.com/v2/";
    pub const OPEN_API_URL: &str = "https://openapi.stage.tidal.com/v2/";
    pub const AUTH_URL: &str = "https://auth.stage.tidal.com/v1/";
    pub const LOGIN_URL: &str = "https://login.stage.tidal.com/";
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Endpoint {  
  OAuth2Token,
  OAuth2DeviceAuth,
  LoginAuthorize,
}
impl Endpoint {
  pub fn to_string(&self) -> String {
    let (base, path) = match self {
      Self::OAuth2Token => (base_urls::AUTH_URL, "oauth2/token"),
      Self::OAuth2DeviceAuth => (base_urls::AUTH_URL, "oauth2/device_authorization"),
      Self::LoginAuthorize => (base_urls::LOGIN_URL, "authorize"),
    };
    format!("{}{}", base, path)
  }
}
