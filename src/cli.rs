use clap::{App, AppSettings, SubCommand};

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
    .subcommand(SubCommand::with_name("list")
      .about("Show the list which files will be managed by dot"))
    .subcommand(SubCommand::with_name("check")
      .about("Check the files are correctly linked to the right places"))
}
