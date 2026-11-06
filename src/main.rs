use crate::args::{Cli, CliCommands};
use chrono::Local;
use clap::Parser;
use std::{env, process};

use timepass::{Entry, TimePass};

mod args;
mod config;

fn format(entry: &Entry) -> Result<String, &'static str> {
  let diff = entry.get_time_passed()?;

  if diff.years == 0 && diff.months == 0 && diff.days == 0 {
    return Ok("added today.".to_string());
  }

  let mut parts = Vec::new();

  if diff.years > 0 {
    let s = if diff.years == 1 { "" } else { "s" };
    parts.push(format!("{} year{}", diff.years, s));
  }

  if diff.months > 0 {
    let s = if diff.months == 1 { "" } else { "s" };
    parts.push(format!("{} month{}", diff.months, s));
  }

  if diff.days > 0 {
    let s = if diff.days == 1 { "" } else { "s" };
    parts.push(format!("{} day{}", diff.days, s));
  }

  match parts.len() {
    1 => Ok(format!("{} passed since.", parts[0])),
    2 => Ok(format!("{} and {} passed since.", parts[0], parts[1])),
    3 => Ok(format!(
      "{}, {}, and {} passed since.",
      parts[0], parts[1], parts[2]
    )),
    _ => unreachable!(),
  }
}

fn display_entry(entry: &Entry) -> Result<(), &'static str> {
  let fmt = format(entry)?;
  println!("{} [{}]\n  - {}\n", entry.name, entry.id, fmt);

  Ok(())
}

fn main() {
  let config = match config::config_init() {
    Ok(path) => path,
    Err(e) => {
      eprintln!("timepass: {}", e);
      process::exit(1);
    }
  };

  let data = config.join("data.json");
  let mut tp = match TimePass::new(data) {
    Ok(v) => v,
    Err(e) => {
      eprintln!("timepass: {}", e);
      process::exit(1);
    }
  };

  if env::args_os().len() == 1 {
    let entries = tp.get_all();
    if entries.is_empty() {
      println!("Nobody here but us chickens!\nTry adding some entries. (timepass add --help)");
    } else {
      for entry in entries.iter() {
        if let Err(e) = display_entry(entry) {
          eprintln!("timepass: Could not display entry: {}", e);
          process::exit(1);
        }
      }
    }

    return;
  }

  let cli = Cli::parse();

  if let Some(name) = cli.name {
    let entries = tp.get_name(&name);
    if entries.is_empty() {
      println!("Nobody here but us chickens!\nTry adding some entries. (timepass add --help)");
    } else {
      for entry in entries.iter() {
        if let Err(e) = display_entry(entry) {
          eprintln!("timepass: Could not display entry: {}", e);
          process::exit(1);
        }
      }
    }
  }

  if let Some(id) = cli.id {
    if let Some(entry) = tp.get_id(&id) {
      if let Err(e) = display_entry(entry) {
        eprintln!("timepass: Could not display entry: {}", e);
        process::exit(1);
      }
    } else {
      println!("Nobody here but us chickens!\nTry adding some entries. (timepass add --help)");
    }
  }

  match cli.command {
    Some(CliCommands::Add { name, date }) => {
      if let Some(value) = date {
        if value > Local::now().date_naive() {
          eprintln!("timepass: The provided date ({}) is in the future.", value);
          std::process::exit(1);
        }
      }

      tp.add(name, date.unwrap_or(Local::now().date_naive()));
    }

    Some(CliCommands::Remove { name, id }) => {}

    None => {}
  }

  if let Err(e) = tp.commit() {
    eprintln!("timepass: {}", e);
    process::exit(1);
  }
}
