use std::collections::HashMap;

use reqwest::{blocking::{Client, RequestBuilder}, Method};

use crate::{auth::{credentials::GrantType, Credentials, TokenResponse}, client::ClientCreds, endpoints::Endpoint, Result};

pub enum RequestAuth<'a> {
  Basic(&'a ClientCreds),
  Bearer(&'a Credentials),
}
impl<'a> From<&'a ClientCreds> for RequestAuth<'a> {
  fn from(creds: &'a ClientCreds) -> Self {
    RequestAuth::Basic(creds)
  }
}
impl<'a> From<&'a Credentials> for RequestAuth<'a> {
  fn from(creds: &'a Credentials) -> Self {
    RequestAuth::Bearer(creds)
  }
}

pub fn request_helper<'a>(client: &Client, method: Method, endpoint: Endpoint, auth: impl Into<RequestAuth<'a>>) -> RequestBuilder {
  let mut builder = client.request(method, endpoint.to_string());
  builder = match auth.into() {
    RequestAuth::Basic(creds) => builder.basic_auth(creds.id(), Some(creds.secret())),
    RequestAuth::Bearer(creds) => builder.bearer_auth(creds.access_token()),
  };

  builder
}
pub fn post_request_helper<'a>(client: &Client, endpoint: Endpoint, auth: impl Into<RequestAuth<'a>>) -> RequestBuilder {
  request_helper(client, Method::POST, endpoint, auth)
}
pub fn get_request_helper<'a>(client: &Client, endpoint: Endpoint, auth: impl Into<RequestAuth<'a>>) -> RequestBuilder {
  request_helper(client, Method::GET, endpoint, auth)
}
pub fn oauth_request_helper<'a>(endpoint: Endpoint, grant: GrantType, auth: impl Into<RequestAuth<'a>>, extra_params: Option<HashMap<&str,&str>>) -> RequestBuilder {
  
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