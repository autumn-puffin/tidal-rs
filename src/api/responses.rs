use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagingResponse<T> {
  pub limit: u64,
  pub offset: u64,
  #[serde(rename = "totalNumberOfItems")]
  pub total_items: u64,
  pub items: Vec<T>,
}
