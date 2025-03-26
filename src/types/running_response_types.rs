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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalResponse {
  pub distance: f64,
  pub running_day: usize,
  pub running_activity: usize,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct PaceGroup {
  pub pace: String,
  pub activity: u32,
  pub distance: f64,
  pub percentage: f64,
}

#[derive(Serialize)]
pub struct PaceGroupResponse {
  pub pace_groups: Vec<PaceGroup>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct DateDistancePaceHr {
  pub date: String,
  pub distance: f64,
  pub pace: String,
  pub hr: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateDistancePaceHrResponse {
  pub table_data: Vec<DateDistancePaceHr>,
}