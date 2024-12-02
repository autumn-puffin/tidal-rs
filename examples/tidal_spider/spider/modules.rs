use super::{Crawl, Target};
use crate::error::Error;
use std::collections::HashSet;
use tidal_rs::api::modules::{collection_modules::*, header_modules::*, *};

impl<T> Crawl for CollectionModule<T>
where
  T: Crawl + Clone,
{
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self
      .paged_list
      .as_ref()
      .map(|l| l.paging.items.clone())
      .unwrap_or_default()
      .identify_targets()
  }
}
impl Crawl for ModuleType {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    match self {
      ModuleType::HighlightModule(highlight_module) => targets.extend(highlight_module.identify_targets()?),
      ModuleType::MixList(mix_module) => targets.extend(mix_module.identify_targets()?),
      ModuleType::MixedTypesList(mixed_list_module) => targets.extend(mixed_list_module.identify_targets()?),
      ModuleType::TrackList(track_module) => targets.extend(track_module.identify_targets()?),
      ModuleType::AlbumList(album_module) => targets.extend(album_module.identify_targets()?),
      ModuleType::PlaylistList(playlist_module) => targets.extend(playlist_module.identify_targets()?),
      ModuleType::PageLinks(page_links_module) => targets.extend(page_links_module.identify_targets()?),
      ModuleType::PageLinksCloud(page_links_cloud_module) => targets.extend(page_links_cloud_module.identify_targets()?),
      ModuleType::AlbumHeader(album_header_module) => targets.extend(album_header_module.identify_targets()?),
      ModuleType::ArtistList(artist_collection_module) => targets.extend(artist_collection_module.identify_targets()?),
      ModuleType::VideoList(video_collection_module) => targets.extend(video_collection_module.identify_targets()?),
      ModuleType::TextBlock(_) => {}
      ModuleType::MultipleTopPromotions(_) => {}
      ModuleType::MixHeader(mix_header_module) => targets.extend(mix_header_module.identify_targets()?),
      ModuleType::ArtistHeader(artist_header_module) => targets.extend(artist_header_module.identify_targets()?),
      ModuleType::AlbumItems(album_items_collection_module) => targets.extend(album_items_collection_module.identify_targets()?),
      ModuleType::ArticleList(_) => {},
      ModuleType::Social(_) => {},
      ModuleType::GenreHeader(genre_header_module) => todo!(),
      ModuleType::ContributorHeader(contributor_header_module) => todo!(),
      ModuleType::FeaturedPromotions => todo!(),
      ModuleType::PageLinksImage => todo!(),
      ModuleType::Radio => todo!(),
      ModuleType::SingleTopPromotion => todo!(),
      ModuleType::Store => todo!(),
      ModuleType::TaskList => todo!(),
      ModuleType::Ticketmaster => todo!(),
      ModuleType::ItemListWithRoles => todo!(),
      ModuleType::LiveSessionList => todo!(),
      ModuleType::Unknown => todo!(),
    }
    Ok(targets)
  }
}
impl Crawl for HighlightModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    for highlight in &self.highlights {
      targets.extend(highlight.item.identify_targets()?);
    }
    Ok(targets)
  }
}
impl Crawl for MixCollectionModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.collection.identify_targets()
  }
}
impl Crawl for AnyMediaCollectionModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.collection.identify_targets()
  }
}
impl Crawl for TrackCollectionModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.collection.identify_targets()
  }
}
impl Crawl for AlbumCollectionModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.collection.identify_targets()
  }
}
impl Crawl for PlaylistCollectionModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.collection.identify_targets()
  }
}
impl Crawl for PageLinksCollectionModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.paged_list.paging.items.identify_targets()
  }
}
impl Crawl for PageLinksCloudCollectionModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.paged_list.paging.items.identify_targets()
  }
}
impl Crawl for AlbumHeaderModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.album.identify_targets()
  }
}
impl Crawl for ArtistCollectionModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.collection.identify_targets()
  }
}
impl Crawl for VideoCollectionModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.collection.identify_targets()
  }
}
impl Crawl for MixHeaderModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.mix.identify_targets()
  }
}
impl Crawl for ArtistHeaderModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets = HashSet::new();
    targets.extend(self.artist.identify_targets()?);
    let mixes = self.mixes.clone();
    for mix in [mixes.artist_mix, mixes.master_artist_mix].iter().flatten() {
      targets.insert(Target::Mix(mix.clone()));
    }
    Ok(targets)
  }
}
impl Crawl for AlbumItemsCollectionModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.collection.identify_targets()
  }
}