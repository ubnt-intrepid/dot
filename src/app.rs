use std::path::Path;
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
    check_symlink_privilege();

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
  if windows::is_user_an_admin() {
    if !windows::is_elevated() {
      panic!("should be elevate as an Administrator.");
    }
  } else {
    if !windows::enable_privilege("SeCreateSymbolicLinkPrivilege") {
      panic!("failed to enable SeCreateSymbolicLinkPrivilege");
    }
  }
}

#[cfg(not(windows))]
#[inline]
pub fn check_symlink_privilege() {}
