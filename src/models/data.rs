use super::entry::Entry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
  pub entries: Vec<Entry>,
}
