use crate::types::running_response_types::DateDistance;

pub fn find_max_daily_distance(date_distance: Vec<DateDistance>) -> f64 {
  date_distance
    .into_iter()
    .map(|item| item.distance)
    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    .unwrap_or(0.0)
}

pub fn round_up_to_nearest_10(n: f64) -> f64 {
  ((n / 10.0).ceil() * 10.0) as f64
}