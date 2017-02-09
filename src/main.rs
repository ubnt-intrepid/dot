extern crate dot;
extern crate clap;

use clap::{Arg, App, AppSettings, SubCommand};
use dot::app;

pub fn main() {
  let retcode = _main();
  std::process::exit(retcode);
}

pub fn _main() -> i32 {
  let matches = cli().get_matches();

  let dotdir = dot::init_envs();
  let app = app::App::new(&dotdir,
                          matches.is_present("dry-run"),
                          matches.is_present("verbose"));


  let retcode = match matches.subcommand() {
    ("check", _) => app.command_check(),
    ("link", _) => app.command_link(),
    ("clean", _) => app.command_clean(),
    ("root", _) => app.command_root(),

    ("clone", Some(args)) => {
      let url = args.value_of("url").unwrap();
      let dotdir = args.value_of("dotdir");
      app.command_clone(url, dotdir)
    }

    ("init", Some(args)) => {
      let url = args.value_of("url").unwrap();
      let dotdir = args.value_of("dotdir");
      let ret = app.command_clone(url, dotdir);
      if ret != 0 {
        return ret;
      }
      app.command_link()
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
    .arg(Arg::with_name("wait-prompt")
      .long("wait-prompt")
      .hidden(true))
    .arg(Arg::with_name("verbose")
      .help("Use verbose output")
      .long("verbose")
      .short("v"))
    .arg(Arg::with_name("dry-run")
      .help("do not actually perform I/O operations")
      .long("dry-run")
      .short("n"))
    .subcommand(SubCommand::with_name("check")
      .about("Check the files are correctly linked to the right places"))
    .subcommand(SubCommand::with_name("link")
      .about("Create all of the symbolic links into home directory"))
    .subcommand(SubCommand::with_name("clean")
      .about("Remote all of registered links from home directory"))
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
        .takes_value(true)))
    .subcommand(SubCommand::with_name("init")
      .about("Clone dotfiles repository from remote & make links")
      .arg(Arg::with_name("url")
        .help("URL of remote repository")
        .required(true)
        .takes_value(true))
      .arg(Arg::with_name("dotdir")
        .help("Path of cloned repository (default: '$DOT_DIR')")
        .takes_value(true)))
    .subcommand(SubCommand::with_name("completion")
      .about("Generate completion scripts")
      .setting(AppSettings::ArgRequiredElseHelp)
      .arg(Arg::with_name("shell")
        .help("target shell")
        .required(true)
        .possible_values(&["bash", "fish", "zsh", "powershell"])))
}
