use std::sync::{mpsc, Arc, Mutex};

use tidal_rs::client::{Auth, Catalogue, DeviceFlow};
use tidal_rs_playground::{AppEvent, BackgroundEvent};

fn main() -> eframe::Result {
  let (background_event_sender, background_event_receiver) = mpsc::channel::<BackgroundEvent>();
  let (app_event_sender, app_event_receiver) = mpsc::channel::<AppEvent>();

  let client = Arc::new(Mutex::new(tidal_rs::client::Client::new(tidal_rs::client::ClientCreds::new(
    dotenvy_macro::dotenv!("ClientID").to_owned(),
    dotenvy_macro::dotenv!("ClientSecret").to_owned(),
  ))));

  let client_mutex = client.clone();
  std::thread::spawn(move || {
    let receiver = background_event_receiver;
    let sender = app_event_sender;
    while let Ok(event) = receiver.recv() {
      use BackgroundEvent::*;
      match event {
        AuthWithDeviceFlow => {
          let mut client = client_mutex.lock().unwrap();
          let dev_res = client.device_login_init().unwrap();
          open::that_in_background(&dev_res.verification_uri_complete);
          client.device_login_finalize(&dev_res).unwrap();
          println!("Logged in as: {:?}\n", client.get_credentials().unwrap().user_id());
        }
        CatalogueGetPage(page) => {
          let client = client_mutex.lock().unwrap();
          let res = client.get_page(&page);
          drop(client);
          match res {
            Ok(response) => {
              sender.send(AppEvent::SetCataloguePage(response)).unwrap();
            }
            Err(e) => println!("Error getting page: {e:?}"),
          }
        }
      }
    }
  });

  let options = eframe::NativeOptions::default();
  eframe::run_native(
    "tidal-rs Playground",
    options,
    Box::new(|_| {
      Ok(Box::new(tidal_rs_playground::App::new(
        background_event_sender,
        app_event_receiver,
        client.clone(),
      )))
    }),
  )
}
