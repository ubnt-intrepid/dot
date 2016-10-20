extern crate clap;

mod cli;

pub fn main() {
  let matches = cli::build_cli().get_matches();

  let exitcode = match matches.subcommand() {
    ("list", Some(m)) => command_list(m),
    (_, _) => unreachable!(),
  };
  std::process::exit(exitcode);
}

// dotlink に登録されているファイルを表示
pub fn command_list(args: &clap::ArgMatches) -> i32 {
  0
}
  
// MEMO:
// std::os::windows::fs::symlink_file("c:\\Users\\xxxx\\.config", "hoge").unwrap();
