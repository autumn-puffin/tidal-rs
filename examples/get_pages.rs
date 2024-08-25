use dotenvy_macro::dotenv;
use tidal_rs::{
  auth::{Auth as _, DeviceFlow as _},
  catalogue::Catalogue,
  client::{Client, ClientCreds},
};

fn get_client() -> Client {
  Client::new(ClientCreds::new(dotenv!("ClientID").to_owned(), dotenv!("ClientSecret").to_owned()))
}

fn auth(client: &mut Client) {
  client.set_country(isocountry::CountryCode::AUS);
  let dev_res = client.device_login_init().unwrap();
  println!("Please Go To https://{}\n", dev_res.verification_uri_complete);
  client.device_login_finalize(&dev_res).unwrap();
  println!("Logged in as: {:?}\n", client.get_credentials().unwrap().user_id());
}

fn main() {
  let mut client = get_client();
  auth(&mut client);
  let res = client.get_explore_page().unwrap();
  println!("Explore Page: {:#?}\n", res);
}
