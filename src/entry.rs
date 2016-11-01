use std::io;
use std::fs;
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
    if !self.src.exists() || self.status() == EntryStatus::Healthy {
      return Ok(()); // Do nothing.
    }

    if self.dst.exists() && !try!(Self::is_symlink(&self.dst)) {
      let origpath = Self::orig_path(&self.dst);
      println!("file {} has already existed. It will be renamed to {}",
               self.dst.display(),
               origpath.display());
      try!(fs::rename(&self.dst, origpath));
    }

    if verbose {
      println!("{}\n  => {}", self.dst.display(), self.src.display());
    }
    util::make_link(&self.src, &self.dst, dry_run)
  }

  pub fn unlink(&self, dry_run: bool, verbose: bool) -> Result<(), io::Error> {
    if !self.dst.exists() || !try!(Self::is_symlink(&self.dst)) {
      return Ok(()); // do nothing
    }

    if verbose {
      println!("unlink {}", self.dst.display());
    }
    try!(util::remove_link(&self.dst, dry_run));

    let origpath = Self::orig_path(&self.dst);
    if origpath.exists() {
      try!(fs::rename(origpath, &self.dst));
    }

    Ok(())
  }

  fn orig_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let origpath = format!("{}.bk", path.as_ref().to_str().unwrap());
    Path::new(&origpath).to_path_buf()
  }

  fn is_symlink<P: AsRef<Path>>(path: P) -> Result<bool, io::Error> {
    let meta = try!(path.as_ref().symlink_metadata());
    Ok(meta.file_type().is_symlink())
  }
}
