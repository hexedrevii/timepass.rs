use crate::args::{Cli, CliCommands};
use chrono::Local;
use clap::Parser;
use std::{env, process};

use timepass::TimePass;

mod args;
mod config;
mod formatting;

fn main() {
  let config = match config::config_init() {
    Ok(path) => path,
    Err(e) => {
      eprintln!("timepass: {e}");
      process::exit(1);
    }
  };

  let data = config.join("data.json");
  let mut tp = match TimePass::new(data) {
    Ok(v) => v,
    Err(e) => {
      eprintln!("timepass: {e}");
      process::exit(1);
    }
  };

  if env::args_os().len() == 1 {
    let entries = tp.get_all();
    formatting::display_entries(&entries);

    return;
  }

  let cli = Cli::parse();

  if let Some(name) = cli.name {
    let entries = tp.get_name(&name);
    formatting::display_entries(&entries);
  }

  if let Some(id) = cli.id {
    if let Some(entry) = tp.get_id(&id) {
      formatting::display_entries(&[entry]);
    } else {
      println!("Nobody here but us chickens!\nTry adding some entries. (timepass add --help)");
    }
  }

  match cli.command {
    Some(CliCommands::Add { name, date }) => {
      if let Some(value) = date
        && value > Local::now().date_naive()
      {
        eprintln!("timepass: The provided date ({value}) is in the future.");
        std::process::exit(1);
      }

      tp.add(name, date.unwrap_or(Local::now().date_naive()));
    }

    Some(CliCommands::Remove {
      name,
      id,
      delete_everything,
    }) => {
      if let Some(value) = name {
        tp.remove_name(&value);
      }

      if let Some(value) = id {
        tp.remove_id(&value);
      }

      if delete_everything {
        tp.remove_all();
      }
    }

    None => {}
  }

  if let Err(e) = tp.commit() {
    eprintln!("timepass: {e}");
    process::exit(1);
  }
}
