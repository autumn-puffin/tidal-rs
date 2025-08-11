use dotenvy_macro::dotenv;
use tidal_rs::client::{Client, ClientCreds};

fn get_client() -> Client {
  Client::new(ClientCreds::new(dotenv!("ClientID").to_owned(), dotenv!("ClientSecret").to_owned()))
}

fn auth(client: &mut Client) {
  let dev_res = client.device_flow_login_init().unwrap();
  println!("Please Go To https://{}\n", dev_res.verification_uri_complete);
  client.device_flow_login_finalize(&dev_res).unwrap();
  println!("Logged in as: {:?}\n", client.get_auth_credentials().unwrap().user_id());
}

// Note: This example requires client credentials capable of device flow (i.e. an android tv client)
fn main() {
  let mut client = get_client();
  auth(&mut client);

  let user = client.get_current_user().unwrap();
  println!("User Info: {:#?}\n", user);

  let sub = client.get_current_user_subscription().unwrap();
  println!("User Subscription: {:#?}\n", sub);

  let clients_res = client.get_current_user_clients().unwrap();
  println!("User Clients: {:#?}\n", clients_res);

  let session = client.get_session_from_auth().unwrap();
  println!("Session Info: {:#?}\n", session);
}
