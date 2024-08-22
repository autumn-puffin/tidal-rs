use dotenvy_macro::dotenv;
use reqwest::Url;

use crate::{auth::{AuthClient, ClientFlow, DeviceFlow, UserFlow}, client::ClientCreds};

fn get_auth() -> AuthClient {
  let mut auth = AuthClient::new(
    // Note: Testing requires setting up a .env file in the root of the project (.env is gitignored)
    ClientCreds::new(dotenv!("ClientID").to_owned(), dotenv!("ClientSecret").to_owned())
  );
  auth.set_redirect_uri(dotenv!("RedirectURI").to_owned());
  auth
}

#[test]
fn test_client_flow() {
  let mut auth = get_auth();
  auth.client_login().unwrap();
}
#[test]
#[ignore]
// Note: This test requires the use of the --nocapture flag as well as the browser
fn test_user_flow() {
  let mut auth = get_auth();
  let response = auth.user_login_init().unwrap();
  println!("Please Go To {:?}\nPaste the redirected url below after logging in\n", response.auth_url);
  let mut code = String::new();
  std::io::stdin().read_line(&mut code).unwrap();
  let code = Url::parse(&code).unwrap().query_pairs().find(|(key, _)| key == "code").unwrap().1.to_string();
  auth.user_login_finalize(code, response).unwrap();
}
#[test]
#[ignore]
// Note: This test requires the use of the --nocapture flag as well as the browser  
fn test_device_flow() {
  let mut auth = get_auth();
  let response = auth.device_login_init().unwrap();
  println!("Please Go To https://{}\n", response.verification_uri_complete);
  auth.device_login_finalize(&response).unwrap();
}