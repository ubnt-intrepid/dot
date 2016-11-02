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
mod dotfiles;
mod entry;
mod util;
#[cfg(windows)]
mod windows;

use std::env;
use clap::{Arg, App, AppSettings, SubCommand};


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

  let matches = cli().get_matches();

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
      cli().gen_completions_to(env!("CARGO_PKG_NAME"),
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

fn cli() -> App<'static, 'static> {
  App::new(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .setting(AppSettings::VersionlessSubcommands)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(SubCommand::with_name("check")
      .about("Check the files are correctly linked to the right places")
      .arg(Arg::with_name("verbose")
        .help("Use verbose output")
        .long("verbose")
        .short("v")))
    .subcommand(SubCommand::with_name("link")
      .about("Create all of the symbolic links into home directory")
      .arg(Arg::with_name("dry-run")
        .help("do not actually perform I/O operations")
        .long("dry-run")
        .short("n"))
      .arg(Arg::with_name("verbose")
        .help("Use verbose output")
        .long("verbose")
        .short("v")))
    .subcommand(SubCommand::with_name("clean")
      .about("Remote all of registered links from home directory")
      .arg(Arg::with_name("dry-run")
        .help("do not actually perform I/O operations")
        .long("dry-run")
        .short("n"))
      .arg(Arg::with_name("verbose")
        .help("Use verbose output")
        .long("verbose")
        .short("v")))
    .subcommand(SubCommand::with_name("root")
      .about("Show the location of dotfiles repository and exit"))
    .subcommand(SubCommand::with_name("clone")
      .about("Clone dotfiles repository from remote")
      .arg(Arg::with_name("url")
        .help("URL of remote repository")
        .required(true)
        .takes_value(true))
      .arg(Arg::with_name("dotdir")
        .help("Path of cloned repository (default: '$DOT_DIR')")
        .takes_value(true))
      .arg(Arg::with_name("dry-run")
        .help("do not actually perform I/O operations")
        .long("dry-run")
        .short("n")))
    .subcommand(SubCommand::with_name("completion")
      .about("Generate completion scripts")
      .setting(AppSettings::ArgRequiredElseHelp)
      .arg(Arg::with_name("shell")
        .help("target shell")
        .required(true)
        .possible_values(&["bash", "fish", "zsh"])))
    .arg(Arg::with_name("wait-prompt")
      .long("wait-prompt")
      .hidden(true))
}
