use crate::{
  client::{
    auth::{AuthCreds, GrantType, TokenResponse},
    ClientCreds,
  },
  endpoints::Endpoint,
  Result,
};
use base64::prelude::*;
use rand::{thread_rng, Rng};
use reqwest::{
  blocking::{Client, RequestBuilder},
  Method,
};
use sha2::{Digest, Sha256};

pub enum RequestAuth<'a> {
  Basic(&'a ClientCreds),
  Bearer(&'a AuthCreds),
}
impl<'a> From<&'a ClientCreds> for RequestAuth<'a> {
  fn from(creds: &'a ClientCreds) -> Self {
    RequestAuth::Basic(creds)
  }
}
impl<'a> From<&'a AuthCreds> for RequestAuth<'a> {
  fn from(creds: &'a AuthCreds) -> Self {
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
pub fn oauth_request_helper<'a>(
  client: &Client,
  endpoint: Endpoint,
  grant: GrantType,
  auth: impl Into<RequestAuth<'a>>,
  extra_params: Option<&[(&str, &str)]>,
) -> RequestBuilder {
  let mut params = extra_params.unwrap_or_default().to_vec();
  params.push(("grant_type", grant.as_str()));

  let mut builder = post_request_helper(client, endpoint, auth);
  builder = builder.form(&params);

  builder
}

pub fn client_login_impl(http_client: &Client, client_credentials: &ClientCreds) -> Result<AuthCreds> {
  let endpoint = Endpoint::OAuth2Token;
  let grant = GrantType::ClientCredentials;

  let res = oauth_request_helper(http_client, endpoint, grant, client_credentials, None).send()?;

  Ok(AuthCreds::new(grant, client_credentials.clone(), res.json::<TokenResponse>()?))
}

/// Generate a new PKCE pair (challenge, verifier)
pub fn new_pkce_pair() -> (String, String) {
  let random: Vec<u8> = (0..32).map(|_| thread_rng().gen::<u8>()).collect();
  let verifier = BASE64_URL_SAFE_NO_PAD.encode(random);

  let digest = Sha256::digest(verifier.as_bytes());
  let challenge = BASE64_URL_SAFE_NO_PAD.encode(digest);

  (challenge, verifier)
}

pub fn res_to_error(res: reqwest::blocking::Response) -> crate::Error {
  let err: crate::error::ApiErrorResponse = res.json().expect("Failed to parse error response");
  match (err.status, err.sub_status, err.error.as_deref()) {
    (401, _, _) => crate::interface::auth::AuthError::Unauthenticated.into(),
    (400, Some(1002), Some("authorization_pending")) => crate::interface::auth::AuthError::AuthorizationPending.into(),
    _ => err.into(),
  }
}
// this is used in tests but linter doesn't see it's used
#[allow(dead_code)]
pub fn client_from_authfile() -> Option<crate::client::Client> {
  let creds = std::fs::read_to_string("./auth.json").ok()?;
  let creds: AuthCreds = serde_json::from_str(&creds).ok()?;
  let country = *creds.country_code().unwrap();
  let mut client = crate::client::Client::new(creds.client_credentials().clone());
  client.set_auth_credentials(creds);
  client.set_country(country);

  Some(client)
}
