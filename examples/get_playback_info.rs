use dotenvy_macro::dotenv;
use tidal_rs::{
  api::{AssetPresentation, AudioQuality, PlaybackInfoOptions, PlaybackMode},
  client::{Client, ClientCreds},
  interface::{auth::*, catalogue::*},
};
use track_catalogue::TrackCatalogue;

fn get_client() -> Client {
  Client::new(ClientCreds::new(dotenv!("ClientID").to_owned(), dotenv!("ClientSecret").to_owned()))
}

fn auth(client: &mut Client) {
  let dev_res = client.device_login_init().unwrap();
  println!("Please Go To https://{}\n", dev_res.verification_uri_complete);
  client.device_login_finalize(&dev_res).unwrap();
  println!("Logged in as: {:?}\n", client.get_credentials().unwrap().user_id());
}

fn main() {
  let mut client = get_client();
  let playback_info_options = PlaybackInfoOptions::new(AudioQuality::HiResLossless, PlaybackMode::Stream, AssetPresentation::Full, false);
  auth(&mut client);
  let playback_info = client.playback_info_for_track(&20115566, &playback_info_options);
  println!("{:#?}", playback_info);
}
