use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceInfo<T> {
  pub trn: String,
  pub item_type: String,
  pub added_at: DateTime<Utc>,
  pub last_modified_at: DateTime<Utc>,
  pub name: Option<String>,
  pub parent: Option<Uuid>,
  pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderData {
  pub trn: String,
  pub item_type: String,
  pub created_at: DateTime<Utc>,
  pub last_modified_at: DateTime<Utc>,
  pub name: Option<String>,
  pub id: Uuid,
  pub total_number_of_items: u64,
}
