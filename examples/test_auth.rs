#[macro_use]
extern crate dotenv_codegen;

use tidal_rs::{auth::{Auth, DeviceFlow}, client::ClientCreds};

fn main() {
    let mut auth = Auth::new(
        ClientCreds::new(dotenv!("ClientID").to_owned(),dotenv!("ClientSecret").to_owned()),
        Some("https://example.com/".to_owned())
    );

    let response = auth.device_login_init().unwrap();
    println!("Please Go To {:?}\n", response.verification_uri_complete);

    auth.device_login_finalize(&response).unwrap();
    let creds = auth.get_credentials().unwrap();
    println!("{:?}", creds);
}
