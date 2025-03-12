use std::collections::HashMap;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Contribution {
  pub date: String,
  pub count: u32,
  pub level: u8,
}

#[derive(Clone, Deserialize, Debug)]
pub struct GithubResponse {
  pub total: HashMap<String, u32>,
  pub contributions: HashMap<u32, HashMap<u32, HashMap<u32, Contribution>>>,
}
