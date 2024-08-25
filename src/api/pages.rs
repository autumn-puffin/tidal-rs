use super::{ModuleType, PagingList};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
  pub self_link: Option<String>,
  pub id: String,
  pub title: String,
  pub rows: Vec<PageRow>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRow {
  pub modules: Vec<PageModule>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageModule {
  pub id: String,
  #[serde(rename = "type")]
  pub module_type: ModuleType,
  pub width: u64,
  pub title: String,
  pub description: String,
  pub pre_title: Option<String>,
  pub show_more: Option<PageItem>,
  pub paged_list: Option<PagingList<PageItem>>,
  pub lines: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageItem {
  pub title: String,
  pub icon: Option<String>,
  pub api_path: String,
  pub image_id: Option<String>,
}
