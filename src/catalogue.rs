use std::{cell::RefCell, rc::Rc};
use isocountry::CountryCode;

use crate::{auth::Credentials, client::ClientCreds};

pub struct Catalogue {
  client_credentials: Rc<ClientCreds>,
  auth_credentials: Option<Rc<RefCell<Credentials>>>,
  country: Option<CountryCode>,
}
impl Catalogue {
  pub fn new(client_credentials: Rc<ClientCreds>) -> Self {
    Self {
      client_credentials,
      auth_credentials: None,
      country: None,
    }
  }
  pub fn set_auth_credentials(&mut self, credentials: Rc<RefCell<Credentials>>) {
    self.auth_credentials = Some(credentials);
  }
  pub fn set_country(&mut self, country: CountryCode) {
    self.country = Some(country);
  }
}