use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paging<T> {
  pub limit: u64,
  pub offset: u64,
  #[serde(rename = "totalNumberOfItems")]
  pub total_items: u64,
  pub items: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagingList<T> {
  #[serde(flatten)]
  pub paging: Paging<T>,
  pub data_api_path: Option<String>,
}
