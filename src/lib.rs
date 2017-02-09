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

pub use app::App;
