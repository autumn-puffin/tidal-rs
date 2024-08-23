use isocountry::CountryCode;

use crate::{
  auth::{Auth, Credentials},
  client::ClientCreds,
};

pub struct CatalogueClient {
  client_credentials: ClientCreds,
  auth_credentials: Option<Credentials>,
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
  pub fn set_auth_credentials(&mut self, credentials: Credentials) {
    self.auth_credentials = Some(credentials);
  }
  pub fn set_country(&mut self, country: CountryCode) {
    self.country = Some(country);
  }
}
pub trait Catalogue: Auth {
  fn get_country(&self) -> Option<&CountryCode>;
}
