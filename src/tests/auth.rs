use crate::client::{Client, ClientCreds};
use dotenvy_macro::dotenv;
use reqwest::Url;

fn get_fresh_client() -> Client {
  let env = (
    dotenv!("ClientID").to_owned(),
    dotenv!("ClientSecret").to_owned(),
    dotenv!("RedirectURI").to_owned(),
  );
  println!("Using id: {}\nUsing secret: {}\nUsing redirect uri: {}", env.0, env.1, env.2);
  let mut client = Client::new(ClientCreds::new(env.0, env.1));
  client.set_redirect_uri(env.2);
  client
}

#[test]
fn test_client_flow() {
  let mut client = get_fresh_client();
  client.client_flow_login().unwrap();
}
#[test]
#[ignore = "requires the --nocapture flag"]
/// Note: This test requires the use of the --nocapture flag as well as the browser
/// `cargo test test_user_flow -- --nocapture --ignored`
fn test_user_flow() {
  let mut client = get_fresh_client();
  let response = client.user_flow_login_init().unwrap();
  println!("Please Go To {:?}\nPaste the redirected url below after logging in\n", response.url());
  let mut code = String::new();
  std::io::stdin().read_line(&mut code).unwrap();
  let code = Url::parse(&code)
    .unwrap()
    .query_pairs()
    .find(|(key, _)| key == "code")
    .unwrap()
    .1
    .to_string();
  client.user_flow_login_finalize(code, response).unwrap();
}
#[test]
#[ignore = "requires the --nocapture flag"]
/// Note: This test requires the use of the --nocapture flag as well as the browser
/// `cargo test test_device_flow -- --nocapture --ignored`
fn test_device_flow() {
  let mut client = get_fresh_client();
  let response = client.device_flow_login_init().unwrap();
  println!("Please Go To https://{}\n", response.verification_uri_complete);
  client.device_flow_login_finalize(&response).unwrap();
}
