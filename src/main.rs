extern crate clap;
extern crate toml;
extern crate shellexpand;
extern crate regex;

mod cli;
mod config;

use config::Config;


pub fn main() {
  let matches = cli::build_cli().get_matches();

  let exitcode = match matches.subcommand() {
    ("list", Some(m)) => command_list(m),
    (_, _) => unreachable!(),
  };
  std::process::exit(exitcode);
}


// dotlink に登録されているファイルを表示
pub fn command_list(_: &clap::ArgMatches) -> i32 {
  let config = Config::new("dotconfig.toml");

  for (linkfile, content) in config.get_linkfiles() {
    println!("Loading {} ...", linkfile);
    for ref entry in content {
      println!("{},{}", entry.source(), entry.dest());
    }
  }

  0
}

// MEMO:
// std::os::windows::fs::symlink_file("c:\\Users\\xxxx\\.config", "hoge").unwrap();
