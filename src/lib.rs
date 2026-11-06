use chrono::NaiveDate;
use serde::Serialize;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub mod models;

pub use models::TimeDifference;
pub use models::data::Data;
pub use models::entry::Entry;

#[derive(Serialize)]
struct DataRef<'a> {
  pub entries: &'a [Entry],
}

pub struct TimePass {
  data: Vec<Entry>,
  path: PathBuf,
}

impl TimePass {
  pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
    if !path.exists() {
      return Ok(Self {
        data: vec![],
        path: path,
      });
    }

    let contents = fs::read_to_string(&path)?;
    let data: Data = serde_json::from_str(&contents)?;
    Ok(Self {
      data: data.entries,
      path: path,
    })
  }

  /// Add an entry to the data vector.
  pub fn add(&mut self, name: String, date: NaiveDate) {
    let entry = Entry {
      id: Uuid::new_v4().to_string(),

      name: name,
      date: date,
    };

    self.data.push(entry);
  }

  pub fn get_all(&self) -> &Vec<Entry> {
    &self.data
  }

  pub fn get_name(&self, name: &str) -> Vec<&Entry> {
    self
      .data
      .iter()
      .filter(|entry| entry.name == name)
      .collect()
  }

  pub fn get_id(&self, id: &str) -> Option<&Entry> {
    self.data.iter().find(|entry| entry.id == id)
  }

  /// Write the data vector to the specified path.
  pub fn commit(&self) -> Result<(), Box<dyn Error>> {
    let data = DataRef {
      entries: &self.data,
    };

    let json = serde_json::to_string_pretty(&data)?;
    fs::write(&self.path, &json)?;

    Ok(())
  }
}
