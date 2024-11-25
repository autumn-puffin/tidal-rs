use crate::{client::{Catalogue as _, RefreshFlow as _, Sessions, Users}, utils::client_from_authfile};

#[test]
fn sessions() {
  let client = client_from_authfile().unwrap();
  let session_from_auth = client.get_session_from_auth().unwrap();
  let session =client.get_session(&session_from_auth.session_id.to_string()).unwrap();
  assert_eq!(session, session_from_auth);
}

#[test]
fn users() {
  let client = client_from_authfile().unwrap();
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
  client.get_mix_page("000b89a1c10b1608a88e4301da7546").unwrap();
  client.get_artist_page(&8847).unwrap();
  client.get_album_page(&80419795).unwrap();
}