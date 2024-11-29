use serde::{Deserialize, Serialize};

pub mod modules;
use modules::ModuleType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
  pub self_link: Option<String>,
  pub id: String,
  pub title: String,
  pub rows: Vec<PageRow>,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRow {
  pub modules: Vec<PageModule>,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PageModule {
  pub id: String,
  #[serde(flatten)]
  pub r#type: ModuleType,
  pub width: u64,
  pub title: Option<String>,
  pub pre_title: Option<String>,
  pub description: Option<String>,
  pub quick_play: bool,
  pub scroll: Option<String>,
  pub self_link: Option<String>,
  pub show_more: Option<ShowMore>,

  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowMore {
  pub title: String,
  pub api_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageItem {
  pub title: Option<String>,
  pub icon: Option<String>,
  pub api_path: Option<String>,
  pub image_id: Option<String>,
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: Box<std::collections::HashMap<String, serde_json::Value>>,
}
