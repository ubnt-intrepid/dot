extern crate ansi_term;
extern crate clap;
extern crate regex;
extern crate shellexpand;
extern crate toml;

mod cli;
mod config;
mod entry;
mod util;

use std::path::Path;
use config::Config;
use entry::EntryStatus;


pub fn main() {
  let ref mut config = Config::new();

  let matches = cli::build_cli().get_matches();
  let dry_run = matches.is_present("dry-run");
  let exitcode = match matches.subcommand() {
    ("list", Some(m)) => command_list(config, m),
    ("check", Some(m)) => command_check(config, m),
    ("clone", Some(m)) => command_clone(config, m, dry_run),
    ("clean", Some(m)) => command_clean(config, m, dry_run),
    ("link", Some(m)) => command_link(config, m, dry_run),
    ("init", Some(m)) => command_init(config, m, dry_run),
    ("pull", Some(m)) => command_pull(config, m, dry_run),
    ("update", Some(m)) => command_update(config, m, dry_run),
    (_, _) => unreachable!(),
  };
  std::process::exit(exitcode);
}

pub fn command_clone(config: &Config, _: &clap::ArgMatches, dry_run: bool) -> i32 {
  let dotdir = util::expand_full(&config.dotdir);
  util::wait_exec("git", &["clone", &config.repo, &dotdir], None, dry_run).unwrap()
}

pub fn command_pull(config: &Config, _: &clap::ArgMatches, dry_run: bool) -> i32 {
  let dotdir = util::expand_full(&config.dotdir);
  util::wait_exec("git", &["pull"], Some(Path::new(&dotdir)), dry_run).unwrap()
}

pub fn command_check(config: &mut Config, _: &clap::ArgMatches) -> i32 {
  let mut num_unhealth = 0;
  for (linkfile, entries) in config.read_linkfiles() {
    println!("{}",
             ansi_term::Style::new()
               .bold()
               .fg(ansi_term::Colour::Blue)
               .paint(format!("Loading {} ...", linkfile)));

    for ref entry in entries {
      let status = entry.status();
      if status != EntryStatus::Healthy {
        println!("{} {} ({:?})",
                 ansi_term::Style::new().bold().fg(ansi_term::Colour::Red).paint("✘"),
                 entry.dst.display(),
                 status);
        num_unhealth += 1;
      } else {
        println!("{} {} (=> {})",
                 ansi_term::Style::new().bold().fg(ansi_term::Colour::Green).paint("✓"),
                 entry.dst.display(),
                 entry.src.display());
      }
    }
  }

  if num_unhealth == 0 { 0 } else { 1 }
}

pub fn command_list(config: &mut Config, _: &clap::ArgMatches) -> i32 {
  for (linkfile, content) in config.read_linkfiles() {
    println!("{}",
             ansi_term::Style::new()
               .bold()
               .fg(ansi_term::Colour::Blue)
               .paint(format!("Loading {} ...", linkfile)));

    for ref entry in content {
      println!("  {} => {}", entry.dst.display(), entry.src.display());
    }
  }

  0
}

pub fn command_link(config: &mut Config, _: &clap::ArgMatches, dry_run: bool) -> i32 {
  for (linkfile, content) in config.read_linkfiles() {
    println!("{}",
             ansi_term::Style::new()
               .bold()
               .fg(ansi_term::Colour::Blue)
               .paint(format!("Loading {} ...", linkfile)));

    for ref entry in content {
      if entry.status() == EntryStatus::Healthy {
        continue;
      }
      println!("link {} => {}", entry.src.display(), entry.dst.display());
      util::make_link(&entry.src, &entry.dst, dry_run).unwrap();
    }
  }

  0
}

pub fn command_clean(config: &mut Config, _: &clap::ArgMatches, dry_run: bool) -> i32 {
  for (linkfile, content) in config.read_linkfiles() {
    println!("{}",
             ansi_term::Style::new()
               .bold()
               .fg(ansi_term::Colour::Blue)
               .paint(format!("Loading {} ...", linkfile)));

    for ref entry in content {
      println!("unlink {}", entry.dst.display());
      util::remove_link(&entry.dst, dry_run).unwrap();
    }
  }

  0
}

pub fn command_init(config: &mut Config, args: &clap::ArgMatches, dry_run: bool) -> i32 {
  let e1 = command_clone(config, args, dry_run);
  if e1 != 0 {
    return e1;
  }
  command_link(config, args, dry_run)
}

pub fn command_update(config: &mut Config, args: &clap::ArgMatches, dry_run: bool) -> i32 {
  let e1 = command_pull(config, args, dry_run);
  if e1 != 0 {
    return e1;
  }
  command_link(config, args, dry_run)
}
