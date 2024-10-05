use std::sync::{Arc, Mutex};

use tidal_rs::client::{Auth, DeviceFlow};
use tidal_rs_playground::Event;

fn main() -> eframe::Result {
  let (event_sender, event_receiver) = std::sync::mpsc::channel::<Event>();
  let client = Arc::new(Mutex::new(tidal_rs::client::Client::new(tidal_rs::client::ClientCreds::new(
    dotenvy_macro::dotenv!("ClientID").to_owned(),
    dotenvy_macro::dotenv!("ClientSecret").to_owned(),
  ))));

  let client_mutex = client.clone();
  std::thread::spawn(move || {
    while let Ok(event) = event_receiver.recv() {
      match event {
        Event::AuthWithDeviceFlow => {
          let mut client = client_mutex.lock().unwrap();
          let dev_res = client.device_login_init().unwrap();
          println!("Please Go To https://{}\n", dev_res.verification_uri_complete);
          client.device_login_finalize(&dev_res).unwrap();
          println!("Logged in as: {:?}\n", client.get_credentials().unwrap().user_id());
        }
      }
    }
  });

  let options = eframe::NativeOptions::default();
  eframe::run_native(
    "tidal-rs Playground",
    options,
    Box::new(|_| Ok(Box::new(tidal_rs_playground::App::new(event_sender, client.clone())))),
  )
}
