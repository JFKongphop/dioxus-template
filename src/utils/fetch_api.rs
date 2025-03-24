use crate::types::running_response_types::{DateDistance, DateDistanceResponse};
use reqwest::Client;

pub async fn get_month_daily_distance() -> Result<Vec<DateDistance>, reqwest::Error> {
  let url = "https://running-api-1007595656483.asia-east2.run.app/api/v1/month-day-distance?year_month=2568-02";

  let response = Client::new()
    .get(url)
    .send()
    .await?
    .error_for_status()?
    .json::<DateDistanceResponse>()
    .await?;

  let date_distance = response.date_distance;

  Ok(date_distance)
}
