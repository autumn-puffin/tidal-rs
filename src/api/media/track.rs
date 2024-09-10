use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
  #[cfg(feature = "show_unmodeled")]
  #[serde(flatten)]
  pub unserialized: std::collections::HashMap<String, serde_json::Value>,
}
