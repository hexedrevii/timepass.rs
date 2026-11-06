use std::path::PathBuf;

use chrono::NaiveDate;
use clap::{Parser, Subcommand, builder::TypedValueParser};

/// Helper to parse from (YYYY/MM/DD) to (DD/MM/YYYY)
fn parse_date(arg: &str) -> Result<NaiveDate, String> {
  NaiveDate::parse_from_str(arg, "%d/%m/%Y")
    .map_err(|e| format!("Dates must use DD/MM/YYYY format. Error: {e}"))
}

/// Helper to validate export path.
fn parse_export(path_str: &str) -> Result<PathBuf, String> {
  let path = PathBuf::from(path_str);

  if path.is_dir() {
    return Err(format!("'{}' is a directory.", path.display()));
  }

  if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
    return Err(format!("Export path must be a JSON file."));
  }

  if let Some(parent) = path.parent() {
    if parent.as_os_str().is_empty() {
      return Ok(path);
    }

    if parent.exists() && !parent.is_dir() {
      return Err(format!(
        "'{}' exists but is not a directory.",
        parent.display()
      ));
    }

    if !parent.exists() {
      return Err(format!(
        "The directory '{}' does not exist.",
        parent.display()
      ));
    }
  }

  Ok(path)
}

#[derive(Parser)]
pub struct Cli {
  #[arg(
    long,
    short,
    conflicts_with = "id",
    help = "Show specific entry via name."
  )]
  pub name: Option<String>,

  #[arg(long, short, help = "Show specific entry via ID.")]
  pub id: Option<String>,

  #[command(subcommand)]
  pub command: Option<CliCommands>,
}

#[derive(Subcommand)]
pub enum CliCommands {
  /// Add a new entry to the system.
  Add {
    #[arg(long, short, help = "The name of the new entry.")]
    name: String,

    #[arg(
      long,
      short,
      help = "The date of purchase (dd/mm/yyyy). Defaults to today if not set.",
      value_parser = parse_date
    )]
    date: Option<NaiveDate>,
  },

  /// Options for importing or exporting data.
  Data {
    #[arg(
      long,
      short,
      required_unless_present = "export",
      conflicts_with = "export",
      value_parser = clap::builder::PathBufValueParser::new().try_map(|path| {
        if path.exists() && path.is_file() {
          Ok(path)
        } else {
          Err("File does not exist.")
        }
      }),
      help = "Path to import the data from. (e.g. ~/backup.json)"
    )]
    import: Option<PathBuf>,

    #[arg(long, short, value_parser = parse_export, help = "Path to export the data to. (e.g. ~/backup.json)")]
    export: Option<PathBuf>,
  },

  /// Remove an entry using either Name or ID.
  Remove {
    #[arg(
      long,
      short,
      required_unless_present_any = ["id", "delete_everything"],
      conflicts_with_all = ["id", "delete_everything"],
      help = "Remove entry via name."
    )]
    name: Option<String>,

    #[arg(long, short, help = "Remove entry via ID.", conflicts_with_all = ["name", "delete_everything"])]
    id: Option<String>,

    #[arg(
      long = "delete-everything-yes-i-am-very-sure",
      help = "Erase all the data in the system."
    )]
    delete_everything: bool,
  },
}
