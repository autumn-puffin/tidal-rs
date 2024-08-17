use std::collections::HashMap;

use reqwest::{blocking::{Client, RequestBuilder}, Method};

use crate::{auth::credentials::GrantType, client::ClientCreds, endpoints::Endpoint};

pub fn request_helper(client: &Client, method: Method, endpoint: Endpoint, auth: &ClientCreds) -> RequestBuilder {
  let mut builder = client.request(method, endpoint.to_string());
  builder = builder.basic_auth(auth.id(), Some(auth.secret()));

  builder
}
pub fn post_request_helper(client: &Client, endpoint: Endpoint, auth: &ClientCreds) -> RequestBuilder {
  request_helper(client, Method::POST, endpoint, auth)
}
pub fn oauth_request_helper(endpoint: Endpoint, grant: GrantType, auth: &ClientCreds, extra_params: Option<HashMap<&str,&str>>) -> RequestBuilder {
  
  let mut params = extra_params.unwrap_or_default();
  params.insert("grant_type", grant.as_str());
  
  let mut builder = post_request_helper(&Client::new(), endpoint, auth);
  builder = builder.form(&params);

  builder
}