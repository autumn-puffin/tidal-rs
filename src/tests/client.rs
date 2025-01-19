use crate::{
  client::{
    album_catalogue::AlbumCatalogue as _, artist_catalogue::ArtistCatalogue as _, video_catalogue::VideoCatalogue as _, Catalogue as _,
    RefreshFlow as _, Sessions, TrackCatalogue, Users,
  },
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
  let session = client.get_session(&session_from_auth.id().to_string()).unwrap();
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

#[test]
fn artists() {
  let artist_id = &EXAMPLE_ARTIST_ID;
  let mut client = client_from_authfile().unwrap();
  client.refresh().unwrap();
  client.get_artist(artist_id).unwrap();
  client.get_artist_bio(artist_id).unwrap();
  client.get_artist_mix_id(artist_id).unwrap();
  client.get_artist_top_tracks(artist_id, &0, &10).unwrap();
  client.get_artist_videos(artist_id, &0, &10).unwrap();
  client.get_artist_albums(artist_id, &0, &10).unwrap();
}

#[test]
fn albums() {
  let album_id = &EXAMPLE_ALBUM_ID;
  let mut client = client_from_authfile().unwrap();
  client.refresh().unwrap();
  client.get_album(album_id).unwrap();
  client.get_album_credits(album_id, true).unwrap();
  client.get_album_items(album_id, &0, &10).unwrap();
  client.get_album_items_with_credits(album_id, &0, &10, true).unwrap();
}
