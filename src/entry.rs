use std::io;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use ansi_term;
use util;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryStatus {
  Healthy,
  LinkNotCreated,
  NotSymLink,
  WrongLinkPath,
}


#[derive(Debug, Clone)]
pub struct Entry {
  src: PathBuf,
  dst: PathBuf,
}

impl Entry {
  pub fn new(src: &str, dst: &str) -> Entry {
    let src = src.replace("/", &format!("{}", MAIN_SEPARATOR));
    let dst = dst.replace("/", &format!("{}", MAIN_SEPARATOR));
    Entry {
      src: Path::new(&src).to_path_buf(),
      dst: Path::new(&dst).to_path_buf(),
    }
  }

  pub fn status(&self) -> EntryStatus {
    if !self.dst.exists() {
      return EntryStatus::LinkNotCreated;
    }

    if !self.dst.symlink_metadata().unwrap().file_type().is_symlink() {
      return EntryStatus::NotSymLink;
    }

    if self.src == self.dst.read_link().unwrap() {
      EntryStatus::Healthy
    } else {
      EntryStatus::WrongLinkPath
    }
  }

  pub fn check(&self, verbose: bool) -> Result<bool, io::Error> {
    let status = self.status();
    if status != EntryStatus::Healthy {
      println!("{} {} ({:?})",
               ansi_term::Style::new().bold().fg(ansi_term::Colour::Red).paint("✘"),
               self.dst.display(),
               status);
      return Ok(false);
    }
    if verbose {
      println!("{} {}\n  => {}",
               ansi_term::Style::new().bold().fg(ansi_term::Colour::Green).paint("✓"),
               self.dst.display(),
               self.src.display());
    }
    Ok(true)
  }

  pub fn mklink(&self, dry_run: bool, verbose: bool) -> Result<(), io::Error> {
    if self.status() == EntryStatus::Healthy {
      if verbose {
        println!("{}\n  the link has already existed.", self.dst.display());
      }
      return Ok(());
    }
    if verbose {
      println!("{}\n  => {}", self.dst.display(), self.src.display());
    }
    util::make_link(&self.src, &self.dst, dry_run)
  }

  pub fn unlink(&self, dry_run: bool, verbose: bool) -> Result<(), io::Error> {
    if verbose {
      println!("unlink {}", self.dst.display());
    }
    util::remove_link(&self.dst, dry_run)
  }
}
