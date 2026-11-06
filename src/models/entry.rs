use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::TimeDifference;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
  pub id: String,
  pub name: String,
  pub date: NaiveDate,
}

impl Entry {
  pub fn get_time_passed(&self) -> Result<TimeDifference, &'static str> {
    let now = Local::now().date_naive();
    let past = self.date;

    let mut years = now.year() - past.year();
    let mut months = now.month() as i32 - past.month() as i32;
    let mut days = now.day() as i32 - past.day() as i32;

    if days < 0 {
      months -= 1;
      let current = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
        .ok_or("Failed to calculate the last day of the current month")?;

      let previous = current
        .pred_opt()
        .ok_or("Failed to calculate the last day of the previous month.")?;

      days += previous.day() as i32;
    }

    if months < 0 {
      years -= 1;
      months += 12;
    }

    Ok(TimeDifference::new(years, months, days))
  }
}
