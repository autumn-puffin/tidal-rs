use std::rc::Rc;

use crate::{auth::Auth, catalogue::Catalogue};

pub struct Client {
  credentials: Rc<ClientCreds>,

  auth: Auth,
  catalogue: Catalogue,
}
impl Client {
  pub fn new(client_credentials: ClientCreds) -> Self {
    let credentials = Rc::new(client_credentials);
    let auth = Auth::new(credentials.clone(), None);
    let catalogue = Catalogue::new(credentials.clone());

    Self {
      credentials,
      auth,
      catalogue,
    }
  }
  pub fn auth(&self) -> &Auth {
    &self.auth
  }
  pub fn catalogue(&self) -> &Catalogue {
    &self.catalogue
  }
}

pub struct ClientCreds {
  client_id: String,
  client_secret: String,
}
impl ClientCreds {
  pub fn new(client_id: String, client_secret: String) -> Self {
    Self { client_id, client_secret }
  }
  pub fn id(&self) -> &str {
    &self.client_id
  }
  pub fn secret(&self) -> &str {
    &self.client_secret
  }
  pub fn as_tuple(&self) -> (&str, &str) {
    (&self.client_id, &self.client_secret)
  }
}
impl std::fmt::Debug for ClientCreds {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ClientCreds")
      .field("client_id", &self.client_id)
      .field("client_secret", &"[REDACTED]")
      .finish()
  }
}
