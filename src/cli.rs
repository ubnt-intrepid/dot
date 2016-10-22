use clap::{Arg, App, AppSettings, SubCommand};

fn build_cli_base() -> App<'static, 'static> {
  App::new(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .setting(AppSettings::VersionlessSubcommands)
    .setting(AppSettings::SubcommandRequiredElseHelp)
}

pub fn build_cli() -> App<'static, 'static> {
  build_cli_base()
    .subcommand(SubCommand::with_name("check")
      .about("Check the files are correctly linked to the right places"))
    .subcommand(SubCommand::with_name("link")
      .about("Create all of the symbolic links into home directory"))
    .subcommand(SubCommand::with_name("clean")
      .about("Remote all of registered links from home directory"))
    .arg(Arg::with_name("dry-run")
      .help("do not actually perform I/O operations")
      .long("dry-run")
      .short("n"))
    .arg(Arg::with_name("verbose")
      .help("Use verbose output")
      .long("verbose")
      .short("v"))
}
