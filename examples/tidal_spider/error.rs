#[derive(Debug)]
pub enum Error {
  Io(std::io::Error),
  SerdeJson(serde_json::Error),
  Reqwest(reqwest::Error),
  Tidal(tidal_rs::Error),
}
impl From<std::io::Error> for Error {
  fn from(e: std::io::Error) -> Self {
    Self::Io(e)
  }
}
impl From<serde_json::Error> for Error {
  fn from(e: serde_json::Error) -> Self {
    Self::SerdeJson(e)
  }
}
impl From<reqwest::Error> for Error {
  fn from(e: reqwest::Error) -> Self {
    Self::Reqwest(e)
  }
}
impl From<tidal_rs::Error> for Error {
  fn from(e: tidal_rs::Error) -> Self {
    Self::Tidal(e)
  }
}
