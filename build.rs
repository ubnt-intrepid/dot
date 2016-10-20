extern crate clap;

use std::path::Path;
use std::fs::OpenOptions;
use clap::Shell;

include!("src/cli.rs");

fn main() {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .open(Path::new(env!("CARGO_MANIFEST_DIR")).join(concat!("_", env!("CARGO_PKG_NAME"))).to_str().unwrap())
    .unwrap();

  build_cli().gen_completions_to(env!("CARGO_PKG_NAME"), Shell::Bash, &mut file);
}
