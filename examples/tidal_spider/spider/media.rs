use super::{Crawl, Target};
use crate::error::Error;
use std::collections::HashSet;
use tidal_rs::api::media::*;

impl Crawl for MediaType {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    match self {
      MediaType::Album(album) => album.identify_targets(),
      MediaType::Artist(artist) => artist.identify_targets(),
      MediaType::Mix(mix) => mix.identify_targets(),
      MediaType::Playlist(playlist) => playlist.identify_targets(),
      MediaType::Profile(_) => Ok(HashSet::new()),
      MediaType::Track(track) => track.identify_targets(),
      MediaType::Video(_) => Ok(HashSet::new()),
    }
  }
}
impl Crawl for Album {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    targets.insert(Target::Album(self.id));
    targets.extend(self.artists.identify_targets()?);
    Ok(targets)
  }
}
impl Crawl for Artist {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    if let Some(id) = self.id {
      targets.insert(Target::Artist(id));
    }
    let mixes = self.mixes.clone();
    for mix in [mixes.artist_mix, mixes.master_artist_mix].iter().flatten() {
      targets.insert(Target::Mix(mix.clone()));
    }

    Ok(targets)
  }
}
impl Crawl for Mix {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    targets.insert(Target::Mix(self.id.clone()));
    Ok(targets)
  }
}
impl Crawl for Playlist {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    targets.insert(Target::Playlist(self.uuid));
    targets.extend(self.promoted_artists.identify_targets()?);
    Ok(targets)
  }
}
impl Crawl for Track {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    targets.insert(Target::Track(self.id));
    targets.extend(self.artists.identify_targets()?);
    targets.extend(self.album.identify_targets()?);
    let mixes = self.mixes.clone();
    for mix in [mixes.track_mix, mixes.master_track_mix].iter().flatten() {
      targets.insert(Target::Mix(mix.clone()));
    }

    Ok(targets)
  }
}
impl Crawl for Video {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    targets.insert(Target::Video(self.id));
    targets.extend(self.artists.identify_targets()?);
    targets.extend(self.album.identify_targets()?);
    Ok(targets)
  }
}
impl Crawl for MediaItem {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    match self {
      MediaItem::Track(track) => track.identify_targets(),
      MediaItem::Video(video) => video.identify_targets(),
    }
  }
}