use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateDistance {
  pub date: String,
  pub distance: f64,
}


#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateDistancePercentage {
  pub date: String,
  pub distance: f64,
  pub percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateDistanceResponse {
  pub date_distance: Vec<DateDistance>,
}