use super::{Crawl, Target};
use crate::error::Error;
use std::collections::HashSet;
use tidal_rs::api::pages::*;

impl Crawl for Page {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.rows.identify_targets()
  }
}
impl Crawl for PageRow {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    self.modules.identify_targets()
  }
}
impl Crawl for PageModule {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    targets.extend(self.r#type.identify_targets()?);
    targets.extend(self.show_more.identify_targets()?);
    Ok(targets)
  }
}
impl Crawl for PageItem {
  fn identify_targets(&self) -> Result<HashSet<Target>, Error> {
    let mut targets: HashSet<Target> = HashSet::new();
    if let Some(api_path) = &self.api_path {
      let path = api_path.replacen("pages/", "", 1);
      targets.insert(Target::Page(path));
    }
    Ok(targets)
  }
}
