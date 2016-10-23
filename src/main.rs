extern crate ansi_term;
extern crate clap;
extern crate shellexpand;
extern crate toml;

mod cli;
mod dotfiles;
mod entry;
mod util;

use std::env;
use std::path::Path;
use dotfiles::Dotfiles;


pub fn main() {
  if env::var("HOME").is_err() {
    env::set_var("HOME", env::home_dir().unwrap().to_str().unwrap());
  }

  let mut app = App::new();

  let matches = cli::build_cli().get_matches();
  let exitcode = app.process_command(matches);

  std::process::exit(exitcode);
}


struct App {
  dotfiles: Dotfiles,
}

impl App {
  pub fn new() -> App {
    let dotdir = env::var("DOT_DIR").expect("$DOT_DIR is not set.");
    let dotdir = Path::new(&dotdir).to_path_buf();
    env::set_var("dotdir", dotdir.as_os_str());

    let dotfiles = Dotfiles::new(dotdir);

    App { dotfiles: dotfiles }
  }

  pub fn process_command(&mut self, matches: clap::ArgMatches) -> i32 {
    match matches.subcommand() {
      ("check", Some(m)) => self.command_check(m),
      ("link", Some(m)) => self.command_link(m),
      ("clean", Some(m)) => self.command_clean(m),
      ("dir", _) => self.command_dir(),
      (_, _) => unreachable!(),
    }
  }

  fn command_dir(&self) -> i32 {
    println!("{}", self.dotfiles.root_dir().display());
    0
  }

  fn command_check(&self, args: &clap::ArgMatches) -> i32 {
    let verbose = args.is_present("verbose");

    let mut num_unhealth = 0;
    for entry in self.dotfiles.entries() {
      if entry.check(verbose).unwrap() == false {
        num_unhealth += 1;
      }
    }

    num_unhealth
  }

  fn command_link(&self, args: &clap::ArgMatches) -> i32 {
    let dry_run = args.is_present("dry-run");
    let verbose = args.is_present("verbose");

    for entry in self.dotfiles.entries() {
      entry.mklink(dry_run, verbose).unwrap();
    }

    0
  }

  fn command_clean(&self, args: &clap::ArgMatches) -> i32 {
    let dry_run = args.is_present("dry-run");
    let verbose = args.is_present("verbose");

    for entry in self.dotfiles.entries() {
      entry.unlink(dry_run, verbose).unwrap();
    }

    0
  }
}
