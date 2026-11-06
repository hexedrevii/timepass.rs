use directories::ProjectDirs;
use std::{fs, path::PathBuf};

pub fn config_init() -> Result<PathBuf, String> {
  let dirs = ProjectDirs::from("", "", "timepass").ok_or("Could not determine HOME.")?;

  let conf = dirs.config_dir();
  if !conf.exists() {
    fs::create_dir_all(conf).map_err(|e| format!("Failed to create config directory: {}", e))?;
  }

  Ok(conf.to_path_buf())
}
