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
  let ref mut config = Config::new("dotconfig.toml");

  let matches = cli::build_cli().get_matches();
  let dry_run = matches.is_present("dry-run");
  let exitcode = match matches.subcommand() {
    ("list", Some(m)) => command_list(config, m),
    ("check", Some(m)) => command_check(config, m),
    ("clone", Some(m)) => command_clone(config, m, dry_run),
    ("clean", Some(m)) => command_clean(config, m, dry_run),
    ("link", Some(m)) => command_link(config, m, dry_run),
    ("init", Some(m)) => command_update(config, m, dry_run),
    (_, _) => unreachable!(),
  };
  std::process::exit(exitcode);
}

pub fn command_clone(config: &Config, _: &clap::ArgMatches, dry_run: bool) -> i32 {
  if Path::new(&config.dotdir).exists() {
    println!("the repository already exists");
    return 1;
  }

  let dotdir = util::expand_full(&config.dotdir);

  util::wait_exec("git", &["clone", &config.repo, &dotdir], None, dry_run).unwrap();
  0
}


pub fn command_check(config: &mut Config, _: &clap::ArgMatches) -> i32 {
  config.read_linkfiles();

  let mut num_unhealth = 0;

  for (linkfile, entries) in config.linkfiles.iter() {
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

pub fn command_list(config: &mut Config, _: &clap::ArgMatches) -> i32 {
  config.read_linkfiles();

  for (linkfile, content) in config.linkfiles.iter() {
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

#[cfg(windows)]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<(), std::io::Error> {
  use std::os::windows::fs::symlink_file;
  symlink_file(src, dst)
}

#[cfg(not(windows))]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<(), std::io::Error> {
  use std::os::unix::fs::symlink;
  symlink(src, dst)
}

fn make_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P,
                                             dst: Q,
                                             dry_run: bool)
                                             -> Result<(), std::io::Error> {
  if dry_run {
    println!("make_link({}, {})",
             src.as_ref().display(),
             dst.as_ref().display());
    Ok(())
  } else {
    try!(std::fs::create_dir_all(dst.as_ref().parent().unwrap()));
    symlink(src, dst)
  }
}

fn remove_link<P: AsRef<Path>>(dst: P, dry_run: bool) -> Result<(), std::io::Error> {
  if dry_run {
    println!("fs::remove_file {}", dst.as_ref().display());
    Ok(())
  } else {
    std::fs::remove_file(dst)
  }
}

pub fn command_link(config: &mut Config, _: &clap::ArgMatches, dry_run: bool) -> i32 {
  config.read_linkfiles();

  for (linkfile, content) in config.linkfiles.iter() {
    println!("{}",
             ansi_term::Style::new()
               .bold()
               .fg(ansi_term::Colour::Blue)
               .paint(format!("Loading {} ...", linkfile)));

    for ref entry in content {
      println!("link {} => {}", entry.src.display(), entry.dst.display());
      make_link(&entry.src, &entry.dst, dry_run).unwrap();
    }
  }

  0
}

pub fn command_clean(config: &mut Config, _: &clap::ArgMatches, dry_run: bool) -> i32 {
  config.read_linkfiles();

  for (linkfile, content) in config.linkfiles.iter() {
    println!("{}",
             ansi_term::Style::new()
               .bold()
               .fg(ansi_term::Colour::Blue)
               .paint(format!("Loading {} ...", linkfile)));

    for ref entry in content {
      println!("unlink {}", entry.dst.display());
      remove_link(&entry.dst, dry_run).unwrap();
    }
  }

  0
}

pub fn command_update(config: &mut Config, args: &clap::ArgMatches, dry_run: bool) -> i32 {
  let e1 = command_clone(config, args, dry_run);
  if e1 != 0 {
    return e1;
  }
  command_link(config, args, dry_run)
}
