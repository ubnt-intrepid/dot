use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryStatus {
  Health,
  LinkNotCreated,
  NotSymLink,
  WrongLinkPath,
}


#[derive(Debug, Clone)]
pub struct Entry {
  pub src: PathBuf,
  pub dst: PathBuf,
}

impl Entry {
  pub fn status(&self) -> EntryStatus {
    if !self.dst.exists() {
      return EntryStatus::LinkNotCreated;
    }

    if !self.dst.symlink_metadata().unwrap().file_type().is_symlink() {
      return EntryStatus::NotSymLink;
    }

    if self.src == self.dst.read_link().unwrap() {
      EntryStatus::Health
    } else {
      EntryStatus::WrongLinkPath
    }
  }
}
