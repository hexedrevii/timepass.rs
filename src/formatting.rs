use colored::Colorize;
use std::process;

use timepass::Entry;

fn format_entry_date(entry: &Entry) -> Result<String, &'static str> {
  let diff = entry.time_since()?;

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

pub fn display_entries(entries: &[&Entry]) {
  if entries.is_empty() {
    println!("Nobody here but us chickens!\nTry adding some entries. (timepass add --help)");
    return;
  }

  println!();
  for entry in entries {
    match format_entry_date(entry) {
      Ok(fmt) => {
        println!(
          "{} {}\n  - {}\n",
          entry.name.green().bold(),
          format!("[{}]", entry.id).ansi_color(8),
          fmt.ansi_color(207)
        );
      }

      Err(e) => {
        eprintln!("timepass: Could not display entry: {e}");
        process::exit(1);
      }
    }
  }
}
