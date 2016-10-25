extern crate clap;
extern crate rustc_cfg;

#[path = "src/cli.rs"]
mod cli;

use rustc_cfg::Cfg;

fn main() {
  let out_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions");
  std::fs::create_dir_all(&out_dir).unwrap();

  let pkg_name = env!("CARGO_PKG_NAME");
  cli::build_cli().gen_completions(pkg_name, clap::Shell::Bash, &out_dir);
  cli::build_cli().gen_completions(pkg_name, clap::Shell::Zsh, &out_dir);
  cli::build_cli().gen_completions(pkg_name, clap::Shell::Fish, &out_dir);

  let cfg = Cfg::new(std::env::var_os("TARGET").unwrap()).unwrap();

  use std::io::Write;
  let mut host = std::fs::OpenOptions::new().create(true).write(true).open("host-triplet").unwrap();
  host.write(format!("{}-{}", cfg.target_arch, cfg.target_os).as_bytes()).unwrap();
}
