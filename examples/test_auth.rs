#[macro_use]
extern crate dotenv_codegen;

use tidal_rs::auth::Auth;

fn main() {
    let mut auth = Auth::new(
        dotenv!("ClientID").to_owned(),
        dotenv!("ClientSecret").to_owned(),    
        Some("https://example.com/".to_owned())
    );

    let response = auth.device_login_init().unwrap();
    println!("Please Go To {:?}\n", response.verification_uri_complete);

    let token = auth.device_login_finalize(&response).unwrap();
    println!("{:?}", token);
}
