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
  let playback_info_options = PlaybackInfoOptions::new(AudioQuality::Lossless, PlaybackMode::Stream, AssetPresentation::Full, false);
  let track_id = 1306665;
  auth(&mut client);

  let track = client.get_track(&track_id);
  println!("Track: {:#?}", track);

  let credits = client.get_track_credits(&track_id, &5, true);
  println!("Credits: {:#?}", credits.unwrap());

  let mix_id = client.get_track_mix_id(&track_id);
  println!("MixId: {:#?}", mix_id);

  let lyrics = client.get_track_lyrics(&track_id);
  println!("Lyrics: {:#?}", lyrics.unwrap());

  let recommendations = client.get_track_recommendations(&track_id, &5);
  println!("Recommendations: {:#?}", recommendations.unwrap());

  let playback_info = client.playback_info_for_track(&track_id, &playback_info_options);
  println!("PlaybackInfo: {:#?}", playback_info);
}
