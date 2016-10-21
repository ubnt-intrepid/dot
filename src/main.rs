extern crate ansi_term;
extern crate clap;
extern crate regex;
extern crate shellexpand;
extern crate toml;

mod cli;
mod config;
mod entry;

use config::Config;
use entry::EntryStatus;


pub fn main() {
  let matches = cli::build_cli().get_matches();

  let exitcode = match matches.subcommand() {
    ("check", Some(m)) => command_check(m),
    ("clean", Some(m)) => command_clean(m),
    ("link", Some(m)) => command_link(m),
    ("list", Some(m)) => command_list(m),
    (_, _) => unreachable!(),
  };
  std::process::exit(exitcode);
}


pub fn command_check(_: &clap::ArgMatches) -> i32 {
  let config = Config::new("dotconfig.toml");

  let mut num_unhealth = 0;

  for (linkfile, entries) in config.linkfiles {
    println!("{}",
             ansi_term::Style::new()
               .bold()
               .fg(ansi_term::Colour::Blue)
               .paint(format!("Loading {} ...", linkfile)));

    for ref entry in entries {
      let status = entry.status();
      if status != EntryStatus::Health {
        println!("{} {} ({:?})",
                 ansi_term::Style::new().bold().fg(ansi_term::Colour::Red).paint("✘"),
                 entry.src.display(),
                 status);
        num_unhealth += 1;
      } else {
        println!("{} {} (=> {})",
                 ansi_term::Style::new().bold().fg(ansi_term::Colour::Green).paint("✓"),
                 entry.src.display(),
                 entry.dst.display());
      }
    }
  }

  if num_unhealth == 0 { 0 } else { 1 }
}


pub fn command_clean(args: &clap::ArgMatches) -> i32 {
  let dry_run = args.is_present("dry-run");

  let config = Config::new("dotconfig.toml");

  for (linkfile, content) in config.linkfiles {
    println!("{}",
             ansi_term::Style::new()
               .bold()
               .fg(ansi_term::Colour::Blue)
               .paint(format!("Loading {} ...", linkfile)));

    for ref entry in content {
      println!("unlink {}", entry.dst.display());
      if dry_run {
        println!("fs::remove_file {}", entry.dst.display());
      } else {
        // std::fs::remove_file(entry.dst).unwrap();
      }
    }
  }

  0
}


pub fn command_link(args: &clap::ArgMatches) -> i32 {
  let dry_run = args.is_present("dry-run");

  let config = Config::new("dotconfig.toml");

  for (linkfile, content) in config.linkfiles {
    println!("{}",
             ansi_term::Style::new()
               .bold()
               .fg(ansi_term::Colour::Blue)
               .paint(format!("Loading {} ...", linkfile)));

    for ref entry in content {
      println!("link {} => {}", entry.src.display(), entry.dst.display());
      if dry_run {
        println!("fs::soft_link {}, {}",
                 entry.src.display(),
                 entry.dst.display());
      } else {
        // std::fs::soft_link(entry.src, entry.dst).unwrap();
      }
    }
  }

  0
}


pub fn command_list(_: &clap::ArgMatches) -> i32 {
  let config = Config::new("dotconfig.toml");

  for (linkfile, content) in config.linkfiles {
    println!("{}",
             ansi_term::Style::new()
               .bold()
               .fg(ansi_term::Colour::Blue)
               .paint(format!("Loading {} ...", linkfile)));

    for ref entry in content {
      println!("{} => {}", entry.src.display(), entry.dst.display());
    }
  }

  0
}


// MEMO:
// std::os::windows::fs::symlink_file("c:\\Users\\xxxx\\.config", "hoge").unwrap();
