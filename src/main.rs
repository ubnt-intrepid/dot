extern crate ansi_term;
extern crate clap;
extern crate shellexpand;
extern crate toml;

mod cli;
mod dotfiles;
mod entry;
mod util;

use std::env;
use dotfiles::Dotfiles;


pub fn main() {
  if env::var("HOME").is_err() {
    env::set_var("HOME", env::home_dir().unwrap().to_str().unwrap());
  }

  let matches = cli::build_cli().get_matches();

  let ref mut config = Dotfiles::new();

  let exitcode = match matches.subcommand() {
    ("check", Some(m)) => command_check(config, m),
    ("link", Some(m)) => command_link(config, m),
    ("clean", Some(m)) => command_clean(config, m),
    ("dir", Some(m)) => command_dir(config, m),
    (_, _) => unreachable!(),
  };
  std::process::exit(exitcode);
}


pub fn command_dir(config: &mut Dotfiles, _: &clap::ArgMatches) -> i32 {
  println!("{}", config.root_dir().display());
  0
}

pub fn command_check(config: &mut Dotfiles, args: &clap::ArgMatches) -> i32 {
  let verbose = args.is_present("verbose");

  let mut num_unhealth = 0;
  for entry in config.entries() {
    if entry.check(verbose).unwrap() == false {
      num_unhealth += 1;
    }
  }

  num_unhealth
}

pub fn command_link(config: &mut Dotfiles, args: &clap::ArgMatches) -> i32 {
  let dry_run = args.is_present("dry-run");
  let verbose = args.is_present("verbose");

  for entry in config.entries() {
    entry.mklink(dry_run, verbose).unwrap();
  }

  0
}

pub fn command_clean(config: &mut Dotfiles, args: &clap::ArgMatches) -> i32 {
  let dry_run = args.is_present("dry-run");
  let verbose = args.is_present("verbose");

  for entry in config.entries() {
    entry.unlink(dry_run, verbose).unwrap();
  }

  0
}
