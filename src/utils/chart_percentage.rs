use crate::types::running_response_types::{DateDistance, DateDistancePercentage};

use super::number::{find_max_daily_distance, round_up_to_nearest_10};

pub fn apply_bar_percentage(date_distance: Vec<DateDistance>) -> Vec<DateDistancePercentage> {
  let max_daily_distance = find_max_daily_distance(date_distance.clone());
  let rounded_tenth = round_up_to_nearest_10(max_daily_distance);

  let applied_distance_percentage: Vec<DateDistancePercentage> = date_distance.clone()
    .into_iter()
    .map(|item| {
      let percentage = (item.distance / rounded_tenth) * 100.0;
      DateDistancePercentage {
        date: item.date,
        distance: item.distance,
        percentage
      }
    }).collect();

  applied_distance_percentage
}