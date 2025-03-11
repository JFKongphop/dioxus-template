use crate::types::github_types::{Contribution, GithubResponse};
use chrono::{Datelike, Duration, NaiveDate, Utc};
use reqwest::Client;

fn flatten_contributions(data: &GithubResponse) -> Vec<Contribution> {
  data.contributions
    .values()
    .flat_map(|year| year.values())
    .flat_map(|month| month.values())
    .cloned() 
    .collect()
}

pub async fn github_contribution() -> Result<u32, reqwest::Error> {
  let today = Utc::now().date_naive();

  let today_this_year = today;
  let today_last_year = today.checked_sub_signed(Duration::days(365)).unwrap() + Duration::days(1);

  let this_year = today_this_year.year();
  let last_year = today_last_year.year();

  let url = format!(
    "https://github-contributions-api.jogruber.de/v4/JFKongphop?format=nested&y={}&y={}",
    this_year, last_year
  );

  let response = Client::new()
    .get(url)
    .send()
    .await?
    .error_for_status()?
    .json::<GithubResponse>()
    .await?;

  let contributions = flatten_contributions(&response);
  let filtered_contributions: Vec<Contribution> = contributions
    .into_iter()
    .filter(|c| {
      let contribution_date = NaiveDate::parse_from_str(&c.date, "%Y-%m-%d").unwrap_or(today_last_year);

      contribution_date >= today_last_year && contribution_date <= today_this_year
    })
    .collect();

  let sum_contributions: u32 = filtered_contributions
    .iter()
    .map(|c| c.count)
    .sum();

  Ok(sum_contributions)
}