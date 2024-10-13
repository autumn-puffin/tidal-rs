use tidal_rs::api::Page;

pub enum BackgroundEvent {
  AuthWithDeviceFlow,
  CatalogueGetPage(String),
}

pub enum AppEvent {
  SetCataloguePage(Page),
}
