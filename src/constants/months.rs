use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static DISTANCE_RANGES: Lazy<HashMap<i32, &str>> = Lazy::new(|| {
  HashMap::from([
    (1, "January"),
    (2, "February"),
    (3, "March"),
    (4, "April"),
    (5, "May"),
    (6, "June"),
    (7, "July"),
    (8, "August"),
    (9, "September"),
    (10, "October"),
    (11, "November"),
    (12, "December"),
  ])
});