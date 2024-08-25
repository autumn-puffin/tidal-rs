use crate::{
  api::{PagingResponse, User, UserClient, UserSubscription},
  auth::{
    credentials::GrantType, flows::UserFlowInfo, oauth::pkce, Auth, AuthClient, AuthError, ClientFlow, Credentials, DeviceFlow, RefreshFlow,
    TokenResponse, UserFlow,
  },
  catalogue::CatalogueClient,
  endpoints::Endpoint,
  error::ApiErrorResponse,
  users::Users,
  utils::{client_login_impl, get_request_helper, oauth_request_helper, post_request_helper},
  Result,
};
use isocountry::CountryCode;
use reqwest::blocking::Client as ReqwestClient;
use std::collections::HashMap;
use url::Url;

pub struct Client {
  client_credentials: ClientCreds,
  auth_credentials: Option<Credentials>,
  scopes: Vec<String>,
  redirect_uri: Option<String>,
  country: Option<CountryCode>,
}
impl Client {
  pub fn new(client_credentials: ClientCreds) -> Self {
    let credentials = client_credentials;

    Self {
      client_credentials: credentials,
      auth_credentials: None,
      scopes: Vec::new(),
      redirect_uri: None,
      country: None,
    }
  }
  pub fn as_auth(&self) -> AuthClient {
    AuthClient::new(self.client_credentials.clone())
  }
  pub fn as_catalogue(&self) -> CatalogueClient {
    CatalogueClient::new(self.client_credentials.clone())
  }
  pub fn set_redirect_uri(&mut self, uri: String) {
    self.redirect_uri = Some(uri);
  }
  pub fn set_country(&mut self, country: CountryCode) {
    self.country = Some(country);
  }
}
impl Auth for Client {
  fn get_credentials(&self) -> Result<&Credentials> {
    self.auth_credentials.as_ref().ok_or(AuthError::Unauthenticated.into())
  }
  fn get_credentials_mut(&mut self) -> Result<&mut Credentials> {
    self.auth_credentials.as_mut().ok_or(AuthError::Unauthenticated.into())
  }
}
impl ClientFlow for Client {
  fn client_login(&mut self) -> Result<()> {
    self.auth_credentials = Some(client_login_impl(&self.client_credentials)?);
    Ok(())
  }
}
impl UserFlow for Client {
  fn user_login_init(&self) -> Result<UserFlowInfo> {
    let redirect_uri = self.redirect_uri.as_ref().ok_or(AuthError::Unauthenticated)?;
    let scopes = self.scopes.join(" ");
    let (pkce_challenge, pkce_verifier) = pkce::new_random_sha256();

    let auth_url = Url::parse_with_params(
      &Endpoint::LoginAuthorize.to_string(),
      &[
        ("response_type", "code"),
        ("client_id", self.client_credentials.id()),
        ("redirect_uri", redirect_uri),
        ("scope", &scopes),
        ("code_challenge_method", "S256"),
        ("code_challenge", &pkce_challenge.as_string()),
      ],
    )?
    .to_string();

    Ok(UserFlowInfo { auth_url, pkce_verifier })
  }

  fn user_login_finalize(&mut self, code: String, info: UserFlowInfo) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::AuthorizationCode;
    let client_credentials = &self.client_credentials;
    let redirect_uri = self.redirect_uri.as_ref().ok_or(AuthError::MissingRedirectUri)?;
    let verifier = info.pkce_verifier.as_string();
    let params = HashMap::from([
      ("client_id", self.client_credentials.id()),
      ("code", &code),
      ("redirect_uri", redirect_uri),
      ("code_verifier", &verifier),
    ]);

    let res = oauth_request_helper(endpoint, grant, client_credentials, Some(params)).send()?;
    let credentials = Credentials::new(grant, client_credentials.clone(), res.json::<TokenResponse>()?);
    self.auth_credentials = Some(credentials);
    Ok(())
  }
}
impl DeviceFlow for Client {
  fn device_login_init(&self) -> Result<crate::auth::flows::DeviceFlowResponse> {
    let client = ReqwestClient::new();
    let endpoint = Endpoint::OAuth2DeviceAuth;
    let client_credentials = &self.client_credentials;

    let params = HashMap::from([("scope", "r_usr+w_usr+w_sub"), ("client_id", self.client_credentials.id())]);

    let res = post_request_helper(&client, endpoint, client_credentials).form(&params).send()?;
    Ok(res.json()?)
  }

  fn try_device_login_finalize(&mut self, response: &crate::auth::flows::DeviceFlowResponse) -> Result<()> {
    let endpoint = Endpoint::OAuth2Token;
    let grant = GrantType::DeviceCode;
    let client_credentials = &self.client_credentials;
    let params = HashMap::from([
      ("scope", "r_usr+w_usr+w_sub"),
      ("client_id", client_credentials.id()),
      ("device_code", &response.device_code),
    ]);
    let res = oauth_request_helper(endpoint, grant, client_credentials, Some(params)).send()?;

    if res.status().is_success() {
      self.auth_credentials = Some(Credentials::new(grant, client_credentials.clone(), res.json::<TokenResponse>()?));
    } else {
      return Err(res.json::<ApiErrorResponse>()?.into());
    }
    Ok(())
  }
}
impl RefreshFlow for Client {
  fn refresh(&mut self) -> Result<()> {
    self.auth_credentials.as_mut().ok_or(AuthError::Unauthenticated)?.refresh()
  }
}

impl Users for Client {
  fn get_user(&self, user_id: &u64) -> Result<User> {
    let client = ReqwestClient::new();
    let endpoint = Endpoint::Users(user_id);
    let auth = self.get_credentials()?;

    let res = get_request_helper(&client, endpoint, auth)
      .query(&[("CountryCode", self.country.unwrap().to_string())])
      .send()?;
    Ok(res.json()?)
  }

  fn get_user_subscription(&self, user_id: &u64) -> Result<UserSubscription> {
    let client = ReqwestClient::new();
    let endpoint = Endpoint::UsersSubscription(user_id);
    let auth = self.get_credentials()?;

    let res = get_request_helper(&client, endpoint, auth)
      .query(&[("CountryCode", self.country.unwrap().to_string())])
      .send()?;
    Ok(res.json()?)
  }

  fn get_user_clients(&self, user_id: &u64) -> Result<PagingResponse<UserClient>> {
    let client = ReqwestClient::new();
    let endpoint = Endpoint::UsersClients(user_id);
    let auth = self.get_credentials()?;

    let res = get_request_helper(&client, endpoint, auth)
      .query(&[("CountryCode", self.country.unwrap().to_string())])
      .send()?;
    Ok(res.json()?)
  }
}

#[derive(Clone)]
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
