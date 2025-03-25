use crate::types::running_response_types::{DateDistance, DateDistanceResponse};
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;

async fn fetch_running_data<T>(path: &str) -> Result<T, Error> 
where 
  T: DeserializeOwned
{
  let url = format!("https://running-api-1007595656483.asia-east2.run.app/api/v1{}", path);
  
  Ok(Client::new()
    .get(url)
    .send()
    .await?
    .error_for_status()?
    .json::<T>()    
    .await?
  )
}

pub async fn get_month_daily_distance() -> Result<Vec<DateDistance>, Error> {
  let path = "/month-day-distance?year_month=2568-02";
  let response: DateDistanceResponse = fetch_running_data(path).await?;
  let date_distance = response.date_distance;

  Ok(date_distance)
}
