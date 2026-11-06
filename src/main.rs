use crate::args::Cli;
use clap::Parser;
use std::env;

mod args;

fn main() {
  if env::args_os().len() == 1 {
    println!("TODO: Show every item.");
    return;
  }

  let cli = Cli::parse();
}
