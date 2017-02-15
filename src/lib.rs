extern crate ansi_term;
extern crate shellexpand;
extern crate toml;
#[macro_use]
extern crate error_chain;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate advapi32;
#[cfg(windows)]
extern crate kernel32;

pub mod app;
mod dotfiles;
mod entry;
pub mod util;
#[cfg(windows)]
mod windows;

mod errors {
  error_chain!{
    foreign_links {
      Io(::std::io::Error);
    }
  }
}
pub use errors::*;

pub use app::App;
