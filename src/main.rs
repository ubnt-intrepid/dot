extern crate clap;

mod cli;

pub fn main() {
  let _ = cli::build_cli().get_matches();
}
