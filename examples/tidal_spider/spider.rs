use crate::error::Error;
use std::collections::HashSet;
use uuid::Uuid;

mod media;
mod modules;
mod page;

pub trait Crawl {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error>;
}

impl<T> Crawl for Vec<T>
where
  T: Crawl,
{
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    for item in self {
      targets.extend(item.identify_targets()?);
    }
    Ok(targets)
  }
}
impl<T> Crawl for Option<T>
where
  T: Crawl,
{
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.as_ref().map_or(Ok(HashSet::new()), |t| t.identify_targets())
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Target {
  Page(String),
  Album(u64),
  Artist(u64),
  Mix(String),
  Playlist(Uuid),
  Profile(u64),
  Track(u64),
  Video(u64),
}
impl std::fmt::Display for Target {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Target::Page(path) => write!(f, "Page: {}", path),
      Target::Album(id) => write!(f, "Album: {}", id),
      Target::Artist(id) => write!(f, "Artist: {}", id),
      Target::Mix(id) => write!(f, "Mix: {}", id),
      Target::Playlist(uuid) => write!(f, "Playlist: {}", uuid),
      Target::Profile(id) => write!(f, "Profile: {}", id),
      Target::Track(id) => write!(f, "Track: {}", id),
      Target::Video(id) => write!(f, "Video: {}", id),
    }
  }
}
