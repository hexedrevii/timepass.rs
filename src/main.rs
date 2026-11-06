use crate::args::Cli;
use chrono::Local;
use clap::Parser;
use std::{env, process};

use timepass::TimePass;

mod args;
mod config;

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
    println!("TODO: Show every item.");
    return;
  }

  let cli = Cli::parse();
}
