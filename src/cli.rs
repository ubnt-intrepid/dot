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
    .arg(Arg::with_name("dry-run")
      .long("dry-run")
      .short("n")
      .help("do not actually perform I/O operations"))
    .subcommand(SubCommand::with_name("list")
      .about("Show the list which files will be managed by dot"))
    .subcommand(SubCommand::with_name("check")
      .about("Check the files are correctly linked to the right places"))
    .subcommand(SubCommand::with_name("clean")
      .about("Remote all of registered links from home directory"))
    .subcommand(SubCommand::with_name("link")
      .about("Create all of the symbolic links into home directory"))
    .subcommand(SubCommand::with_name("clone")
      .about("Clone the repository of dotfiles from remote"))
    .subcommand(SubCommand::with_name("pull").about("Pull from remote repository"))
    .subcommand(SubCommand::with_name("init")
      .about("Perform 'clone' and then 'link' if successfully cloned"))
    .subcommand(SubCommand::with_name("update")
      .about("Perform 'pull' and then 'link' if successfully cloned"))
}
