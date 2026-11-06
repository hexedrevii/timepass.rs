use chrono::NaiveDate;
use clap::{Parser, Subcommand};

/// Helper to parse from (YYYY/MM/DD) to (DD/MM/YYYY)
fn parse_date(arg: &str) -> Result<NaiveDate, String> {
  NaiveDate::parse_from_str(arg, "%d/%m/%Y")
    .map_err(|e| format!("Dates must use DD/MM/YYYY format. Error: {e}"))
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

  /// Remove an entry using either Name or ID.
  Remove {
    #[arg(
      long,
      short,
      required_unless_present = "id",
      conflicts_with = "id",
      help = "Remove entry via name."
    )]
    name: Option<String>,

    #[arg(long, short, help = "Remove entry via ID.")]
    id: Option<String>,
  },
}
