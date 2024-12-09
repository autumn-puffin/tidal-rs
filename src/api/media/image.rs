use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
  pub url: String,
  pub width: u64,
  pub height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Graphic {
  pub images: Vec<Image>,
  pub text: String,
  pub r#type: String,
}
