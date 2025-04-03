use chrono::prelude::*;

pub fn month_number_to_name(year_month: &str) -> Month {
  let parts: Vec<&str> = year_month.split('-').collect();
  let year = parts[0].parse::<i32>().unwrap_or(1);
  let month = parts[1].parse::<u32>().unwrap_or(1);
  let date = Utc.with_ymd_and_hms(
    year, 
    month, 
    0, 
    0, 
    0, 
    0
  ).unwrap();
  let month = Month::try_from(u8::try_from(date.month()).unwrap()).unwrap_or(Month::January);

  month

}