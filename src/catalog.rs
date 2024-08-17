use std::rc::Rc;
use crate::client::ClientCreds;

pub struct Catalog {
  client_credentials: Rc<ClientCreds>,
}