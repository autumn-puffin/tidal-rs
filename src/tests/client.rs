use crate::{
  client::{video_catalogue::VideoCatalogue as _, Catalogue as _, RefreshFlow as _, Sessions, TrackCatalogue, Users},
  utils::client_from_authfile,
};

const EXAMPLE_ARTIST_ID: u64 = 4557268; // Death Grips
const EXAMPLE_MIX_ID: &str = "00011a5335dc6c5c31431ca489f7d6"; // Death Grips Radio
const EXAMPLE_ALBUM_ID: u64 = 14558039; // The Money Store - Death Grips
const EXAMPLE_TRACK_ID: u64 = 14558045; // I've Seen Footage - The Money Store - Death Grips
const EXAMPLE_VIDEO_ID: u64 = 29484637; // Get Got - The Money Store - Death Grips

#[test]
fn sessions() {
  let mut client = client_from_authfile().unwrap();
  client.refresh().unwrap();
  let session_from_auth = client.get_session_from_auth().unwrap();
  let session = client.get_session(&session_from_auth.session_id.to_string()).unwrap();
  assert_eq!(session, session_from_auth);
}

#[test]
fn users() {
  let mut client = client_from_authfile().unwrap();
  client.refresh().unwrap();
  client.get_current_user().unwrap();
  client.get_current_user_subscription().unwrap();
  client.get_current_user_clients().unwrap();
}

#[test]
fn pages() {
  let mut client = client_from_authfile().unwrap();
  client.refresh().unwrap();
  client.get_home_page().unwrap();
  client.get_explore_page().unwrap();
  client.get_mix_page(EXAMPLE_MIX_ID).unwrap();
  client.get_artist_page(&EXAMPLE_ARTIST_ID).unwrap();
  client.get_album_page(&EXAMPLE_ALBUM_ID).unwrap();
}

#[test]
fn tracks() {
  let track_id = &EXAMPLE_TRACK_ID;
  let mut client = client_from_authfile().unwrap();
  client.refresh().unwrap();
  client.get_track(track_id).unwrap();
  client.get_track_credits(track_id, &10, true).unwrap();
  client.get_track_lyrics(track_id).unwrap();
  client.get_track_mix_id(track_id).unwrap();
  client.get_track_recommendations(track_id, &10).unwrap();
}

#[test]
fn videos() {
  let video_id = &EXAMPLE_VIDEO_ID;
  let mut client = client_from_authfile().unwrap();
  client.refresh().unwrap();
  client.get_video(video_id).unwrap();
  client.get_video_recommendations(video_id, &10).unwrap();
}
