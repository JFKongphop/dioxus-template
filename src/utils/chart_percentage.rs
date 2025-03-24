use crate::types::running_response_types::DateDistance;

pub fn ap(date_distance: Option<Vec<DateDistance>>) -> f64 {
  match date_distance {
    Some(distances) => distances
      .into_iter()
      .map(|item| item.distance)
      .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
      .unwrap_or(0.0),
    None => 0.0,
  }
}