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
}
