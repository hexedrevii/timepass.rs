use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
  pub id: String,
  pub name: String,
  pub date: NaiveDate,
}
