pub struct TimeDifference {
  pub years: i32,
  pub months: i32,
  pub days: i32,
}

impl TimeDifference {
  #[must_use]
  pub fn new(years: i32, months: i32, days: i32) -> Self {
    Self {
      years,
      months,
      days,
    }
  }
}

pub mod data;
pub mod entry;
