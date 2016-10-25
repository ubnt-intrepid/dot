use std::env;
use std::path::Path;
use std::process;
use runas;
use dotfiles::Dotfiles;
use util;
#[cfg(windows)]
use windows;

pub struct App {
  dotfiles: Dotfiles,
}

impl App {
  pub fn new(dotdir: &str) -> App {
    let dotfiles = Dotfiles::new(Path::new(dotdir).to_path_buf());
    App { dotfiles: dotfiles }
  }

  pub fn command_clone(&self, url: &str, dotdir: Option<&str>, dry_run: bool) -> i32 {
    let dotdir = dotdir.unwrap_or(self.dotfiles.root_dir().to_str().unwrap());
    util::wait_exec("git", &["clone", url, dotdir], None, dry_run).unwrap()
  }

  pub fn command_root(&self) -> i32 {
    println!("{}", self.dotfiles.root_dir().display());
    0
  }

  pub fn command_check(&self, verbose: bool) -> i32 {
    let mut num_unhealth = 0;
    for entry in self.dotfiles.entries() {
      if entry.check(verbose).unwrap() == false {
        num_unhealth += 1;
      }
    }
    num_unhealth
  }

  pub fn command_link(&self, dry_run: bool, verbose: bool) -> i32 {
    if !dry_run {
      check_symlink_privilege();
    }

    for entry in self.dotfiles.entries() {
      entry.mklink(dry_run, verbose).unwrap();
    }
    0
  }

  pub fn command_clean(&self, dry_run: bool, verbose: bool) -> i32 {
    for entry in self.dotfiles.entries() {
      entry.unlink(dry_run, verbose).unwrap();
    }
    0
  }
}


#[cfg(windows)]
fn check_symlink_privilege() {
  use windows::ElevationType;
  match windows::get_elevation_type().unwrap() {
    ElevationType::Default => {
      if !windows::enable_privilege("SeCreateSymbolicLinkPrivilege") {
        panic!("failed to enable SeCreateSymbolicLinkPrivilege");
      }
    }
    ElevationType::Limited => {
      let mut args = vec!["--wait-prompt".to_owned()];
      args.extend(env::args().skip(1));
      let status = runas::Command::new(env::current_exe().unwrap())
        .args(args.as_slice())
        .show(true)
        .status()
        .unwrap();
      process::exit(status.code().unwrap());
      // panic!("should be elevate as an Administrator.");
    }
    ElevationType::Full => (),
  }
}

#[cfg(not(windows))]
#[inline]
pub fn check_symlink_privilege() {}
