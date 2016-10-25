extern crate clap;

#[path = "src/cli.rs"]
mod cli;

fn main() {
  let out_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions");
  std::fs::create_dir_all(&out_dir).unwrap();

  let pkg_name = env!("CARGO_PKG_NAME");
  cli::build_cli().gen_completions(pkg_name, clap::Shell::Bash, &out_dir);
  cli::build_cli().gen_completions(pkg_name, clap::Shell::Zsh, &out_dir);
  cli::build_cli().gen_completions(pkg_name, clap::Shell::Fish, &out_dir);

  use std::io::Write;
  let mut host = std::fs::OpenOptions::new().create(true).write(true).open("host-triplet").unwrap();
  host.write_fmt(format_args!("{}\n", env!("HOST"))).unwrap();
}
