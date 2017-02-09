extern crate ansi_term;
extern crate shellexpand;
extern crate toml;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate advapi32;
#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate runas;

pub mod app;
mod dotfiles;
mod entry;
pub mod util;
#[cfg(windows)]
mod windows;

use std::env;

pub fn init_envs() -> String {
  if env::var("HOME").is_err() {
    env::set_var("HOME", env::home_dir().unwrap().to_str().unwrap());
  }

  let dotdir = env::var("DOT_DIR").or(util::expand_full("$HOME/.dotfiles")).unwrap();
  env::set_var("DOT_DIR", dotdir.as_str());
  env::set_var("dotdir", dotdir.as_str());

  dotdir
}
