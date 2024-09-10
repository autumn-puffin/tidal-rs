use super::{auth::AuthCreds, ClientCreds};
use isocountry::CountryCode;

pub struct CatalogueClient {
  client_credentials: ClientCreds,
  auth_credentials: Option<AuthCreds>,
  country: Option<CountryCode>,
}
impl CatalogueClient {
  pub fn new(client_credentials: ClientCreds) -> Self {
    Self {
      client_credentials,
      auth_credentials: None,
      country: None,
    }
  }
  pub fn set_auth_credentials(&mut self, credentials: AuthCreds) {
    self.auth_credentials = Some(credentials);
  }
  pub fn set_country(&mut self, country: CountryCode) {
    self.country = Some(country);
  }
}
