use std::{cell::RefCell, rc::Rc};
use crate::{auth::Credentials, client::ClientCreds};

pub struct Catalog {
  client_credentials: Rc<ClientCreds>,
  auth_credentials: Option<Rc<RefCell<Credentials>>>,
}
impl Catalog {
  pub fn new(client_credentials: Rc<ClientCreds>) -> Self {
    Self {
      client_credentials,
      auth_credentials: None,
    }
  }
  pub fn set_auth_credentials(&mut self, credentials: Rc<RefCell<Credentials>>) {
    self.auth_credentials = Some(credentials);
  }
}