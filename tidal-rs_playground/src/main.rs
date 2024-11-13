use std::{
  fs::{self, File},
  io::Write as _,
  sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
  },
};

use tidal_rs::client::{auth::AuthCreds, Auth, Catalogue, Client, ClientCreds, DeviceFlow};
use tidal_rs_playground::{AppEvent, BackgroundEvent};

fn main() -> eframe::Result {
  let (background_event_sender, background_event_receiver) = mpsc::channel::<BackgroundEvent>();
  let (app_event_sender, app_event_receiver) = mpsc::channel::<AppEvent>();

  let client = Arc::new(Mutex::new(init_client()));
  let event_handler_client = client.clone();

  std::thread::spawn(move || event_handler(event_handler_client, background_event_receiver, app_event_sender));

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

fn init_client() -> Client {
  let creds = ClientCreds::new(
    dotenvy_macro::dotenv!("ClientID").to_owned(),
    dotenvy_macro::dotenv!("ClientSecret").to_owned(),
  );
  let mut client = Client::new(creds);
  if let Ok(auth_json) = fs::read_to_string("./auth.json") {
    if let Ok(auth) = serde_json::from_str::<AuthCreds>(&auth_json) {
      client.set_country(*auth.country_code().unwrap());
      client.set_auth_credentials(auth);
    };
  }

  client
}

fn event_handler(client: Arc<Mutex<Client>>, receiver: Receiver<BackgroundEvent>, sender: Sender<AppEvent>) {
  while let Ok(event) = receiver.recv() {
    use BackgroundEvent::*;
    match event {
      AuthWithDeviceFlow => {
        let mut client_locked = client.lock().unwrap();
        let dev_res = client_locked.device_login_init().unwrap();
        open::that_in_background(&dev_res.verification_uri_complete);
        client_locked.device_login_finalize(&dev_res).unwrap();
        println!("Logged in as: {:?}\n", client_locked.get_credentials().unwrap().user_id());
        let auth = client_locked.get_auth_credentials().unwrap();
        let creds_json = serde_json::to_string_pretty(&auth).unwrap();
        let mut file = File::create("auth.json").unwrap();
        file.write_all(creds_json.as_bytes()).unwrap();
      }
      CatalogueGetPage(page) => {
        let mut client_locked = client.lock().unwrap();
        let res = client_locked.map_refresh(|c| c.get_page(&page));
        match res {
          Ok(response) => sender.send(AppEvent::SetCataloguePage(response)).unwrap(),
          Err(e) => println!("Error getting page: {e:?}"),
        }
      }
    }
  }
}
