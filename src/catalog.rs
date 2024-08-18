use std::{cell::RefCell, rc::Rc};
use crate::{auth::Credentials, client::ClientCreds};

pub struct Catalog {
  client_credentials: Rc<ClientCreds>,
  auth_credentials: Option<Rc<RefCell<Credentials>>>,
}