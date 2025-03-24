use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DateDistance {
  pub date: String,
  pub distance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateDistanceResponse {
  pub date_distance: Vec<DateDistance>,
}