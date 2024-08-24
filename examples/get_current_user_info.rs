use dotenvy_macro::dotenv;
use tidal_rs::{
  auth::{Auth as _, DeviceFlow as _},
  client::{Client, ClientCreds},
  users::Users,
};

fn get_client() -> Client {
  Client::new(ClientCreds::new(dotenv!("ClientID").to_owned(), dotenv!("ClientSecret").to_owned()))
}

// Note: This example requires client credentials capable of device flow (i.e. an android tv client)
fn main() {
  let mut client = get_client();
  client.set_country(isocountry::CountryCode::AUS);
  let dev_res = client.device_login_init().unwrap();
  println!("Please Go To https://{}\n", dev_res.verification_uri_complete);
  client.device_login_finalize(&dev_res).unwrap();
  println!("Logged in as: {:?}\n", client.get_credentials().unwrap().user_id());

  let user = client.get_current_user().unwrap();
  println!("User Info: {:#?}\n", user);

  let sub = client.get_current_user_subscription().unwrap();
  println!("User Subscription: {:#?}\n", sub);

  let clients_res = client.get_current_user_clients().unwrap();
  println!("User Clients: {:#?}\n", clients_res);
}
