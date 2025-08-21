use base64::prelude::*;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

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
    (401, _, _) => crate::error::AuthError::Unauthenticated.into(),
    (400, Some(1002), Some("authorization_pending")) => crate::error::AuthError::AuthorizationPending.into(),
    _ => err.into(),
  }
}
// this is used in tests but linter doesn't see it's used
#[allow(dead_code)]
pub fn client_from_authfile() -> Option<crate::client::Client> {
  let creds = std::fs::read_to_string("./auth.json").ok()?;
  let creds: crate::client::AuthCreds = serde_json::from_str(&creds).ok()?;
  let country = *creds.country_code().unwrap();
  let mut client = crate::client::Client::new(creds.client_credentials().clone());
  client.set_auth_credentials(creds);
  client.set_country(country);

  Some(client)
}
