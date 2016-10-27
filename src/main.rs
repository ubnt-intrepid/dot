extern crate ansi_term;
extern crate clap;
extern crate shellexpand;
extern crate toml;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate advapi32;
#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate runas;

mod app;
mod cli;
mod dotfiles;
mod entry;
mod util;
#[cfg(windows)]
mod windows;

use std::env;


pub fn main() {
  let retcode = _main();
  std::process::exit(retcode);
}

pub fn _main() -> i32 {
  if env::var("HOME").is_err() {
    env::set_var("HOME", env::home_dir().unwrap().to_str().unwrap());
  }

  let dotdir = env::var("DOT_DIR").unwrap_or(util::expand_full("$HOME/.dotfiles").unwrap());
  env::set_var("DOT_DIR", dotdir.as_str());
  env::set_var("dotdir", dotdir.as_str());

  let matches = cli::build_cli().get_matches();

  let app = app::App::new(&dotdir);
  let retcode = match matches.subcommand() {
    ("check", Some(args)) => {
      let verbose = args.is_present("verbose");
      app.command_check(verbose)
    }

    ("link", Some(args)) => {
      let dry_run = args.is_present("dry-run");
      let verbose = args.is_present("verbose");
      app.command_link(dry_run, verbose)
    }

    ("clean", Some(args)) => {
      let dry_run = args.is_present("dry-run");
      let verbose = args.is_present("verbose");
      app.command_clean(dry_run, verbose)
    }

    ("root", _) => app.command_root(),

    ("clone", Some(args)) => {
      let url = args.value_of("url").unwrap();
      let dotdir = args.value_of("dotdir");
      let dry_run = args.is_present("dry-run");
      app.command_clone(url, dotdir, dry_run)
    }

    ("completion", Some(args)) => {
      let shell = args.value_of("shell").unwrap();
      cli::build_cli().gen_completions_to(env!("CARGO_PKG_NAME"),
                                          shell.parse::<clap::Shell>().unwrap(),
                                          &mut std::io::stdout());
      0
    }

    (_, _) => unreachable!(),
  };

  if matches.is_present("wait-prompt") {
    println!("press enter to exit...");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().expect("failed to read a line.");
  }

  retcode
}
