use std::collections::HashMap;

use reqwest::{blocking::{Client, RequestBuilder}, Method};

use crate::{auth::{credentials::GrantType, Credentials, TokenResponse}, client::ClientCreds, endpoints::Endpoint, Result};


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

pub fn client_login_impl(client_credentials: &ClientCreds) -> Result<Credentials> {
  let endpoint = Endpoint::OAuth2Token;
  let grant = GrantType::ClientCredentials;

  let res = oauth_request_helper( endpoint, grant, client_credentials, None).send()?;

  Ok(Credentials::new(grant, client_credentials.clone(), res.json::<TokenResponse>()?))
}