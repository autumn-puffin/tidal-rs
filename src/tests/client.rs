use uuid::Uuid;

use crate::utils::client_from_authfile;

const EXAMPLE_ARTIST_ID: u64 = 4557268; // Death Grips
const EXAMPLE_MIX_ID: &str = "00011a5335dc6c5c31431ca489f7d6"; // Death Grips Radio
const EXAMPLE_ALBUM_ID: u64 = 14558039; // The Money Store - Death Grips
const EXAMPLE_TRACK_ID: u64 = 14558045; // I've Seen Footage - The Money Store - Death Grips
const EXAMPLE_VIDEO_ID: u64 = 29484637; // Get Got - The Money Store - Death Grips
const EXAMPLE_UNOWNED_PLAYLIST_ID: Uuid = Uuid::from_u128(0x395140403d8a40d7b4e46edb48b90a42); // The Prodigy Essentials - Tidal

mod sessions {
  use crate::interface::{RefreshFlow, Sessions};

  #[test]
  fn get_session_from_auth() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_session_from_auth().unwrap();
  }
  #[test]
  fn get_session() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    let session_from_auth = client.get_session_from_auth().unwrap();
    client.get_session(&session_from_auth.id().to_string()).unwrap();
  }
}

mod users {
  use crate::interface::{RefreshFlow, Users};

  #[test]
  fn get_current_user() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_current_user().unwrap();
  }
  #[test]
  fn get_current_user_subscription() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_current_user_subscription().unwrap();
  }
  #[test]
  fn get_current_user_clients() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_current_user_clients().unwrap();
  }
}

mod pages {
  use super::{EXAMPLE_ALBUM_ID, EXAMPLE_ARTIST_ID, EXAMPLE_MIX_ID};
  use crate::interface::{Catalogue, RefreshFlow};

  #[test]
  fn get_home_page() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_home_page().unwrap();
  }
  #[test]
  fn get_explore_page() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_explore_page().unwrap();
  }
  #[test]
  fn get_mix_page() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_mix_page(EXAMPLE_MIX_ID).unwrap();
  }
  #[test]
  fn get_artist_page() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_artist_page(&EXAMPLE_ARTIST_ID).unwrap();
  }
  #[test]
  fn get_album_page() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_album_page(&EXAMPLE_ALBUM_ID).unwrap();
  }
}

mod tracks {
  use super::EXAMPLE_TRACK_ID;
  use crate::interface::{RefreshFlow, TrackCatalogue};

  #[test]
  fn get_track() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_track(&EXAMPLE_TRACK_ID).unwrap();
  }
  #[test]
  fn get_track_credits() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_track_credits(&EXAMPLE_TRACK_ID, &10, true).unwrap();
  }
  #[test]
  fn get_track_lyrics() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_track_lyrics(&EXAMPLE_TRACK_ID).unwrap();
  }
  #[test]
  fn get_track_mix_id() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_track_mix_id(&EXAMPLE_TRACK_ID).unwrap();
  }
  #[test]
  fn get_track_recommendations() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_track_recommendations(&EXAMPLE_TRACK_ID, &10).unwrap();
  }
}

mod videos {
  use super::EXAMPLE_VIDEO_ID;
  use crate::interface::{RefreshFlow, VideoCatalogue};

  #[test]
  fn get_video() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_video(&EXAMPLE_VIDEO_ID).unwrap();
  }
  #[test]
  fn get_video_recommendations() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_video_recommendations(&EXAMPLE_VIDEO_ID, &10).unwrap();
  }
}

mod artists {
  use super::EXAMPLE_ARTIST_ID;
  use crate::interface::{ArtistCatalogue, RefreshFlow};

  #[test]
  fn get_artist() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_artist(&EXAMPLE_ARTIST_ID).unwrap();
  }
  #[test]
  fn get_artist_bio() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_artist_bio(&EXAMPLE_ARTIST_ID).unwrap();
  }
  #[test]
  fn get_artist_mix_id() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_artist_mix_id(&EXAMPLE_ARTIST_ID).unwrap();
  }
  #[test]
  fn get_artist_top_tracks() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_artist_top_tracks(&EXAMPLE_ARTIST_ID, &0, &10).unwrap();
  }
  #[test]
  fn get_artist_videos() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_artist_videos(&EXAMPLE_ARTIST_ID, &0, &10).unwrap();
  }
  #[test]
  fn get_artist_albums() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_artist_albums(&EXAMPLE_ARTIST_ID, &0, &10).unwrap();
  }
}

mod albums {
  use super::EXAMPLE_ALBUM_ID;
  use crate::interface::{AlbumCatalogue, RefreshFlow};

  #[test]
  fn get_album() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_album(&EXAMPLE_ALBUM_ID).unwrap();
  }
  #[test]
  fn get_album_credits() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_album_credits(&EXAMPLE_ALBUM_ID, true).unwrap();
  }
  #[test]
  fn get_album_items() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_album_items(&EXAMPLE_ALBUM_ID, &0, &10).unwrap();
  }
  #[test]
  fn get_album_items_with_credits() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_album_items_with_credits(&EXAMPLE_ALBUM_ID, &0, &10, true).unwrap();
  }
}

mod playlists {
  use super::EXAMPLE_UNOWNED_PLAYLIST_ID;
  use crate::interface::{PlaylistCatalogue, RefreshFlow};

  #[test]
  fn get_playlist() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_playlist(&EXAMPLE_UNOWNED_PLAYLIST_ID).unwrap();
  }
  #[test]
  fn get_playlist_items() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_playlist_items(&EXAMPLE_UNOWNED_PLAYLIST_ID, &0, &10).unwrap();
  }
  #[test]
  fn get_playlist_recommendations() {
    let mut client = super::client_from_authfile().unwrap();
    client.refresh().unwrap();
    client.get_playlist_recommendations(&EXAMPLE_UNOWNED_PLAYLIST_ID, &0, &10).unwrap();
  }
}
