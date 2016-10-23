use std::path::{Path, PathBuf};
use entry::Entry;
use util;

pub struct Dotfiles {
  _root_dir: PathBuf,
  _entries: Vec<Entry>,
}

impl Dotfiles {
  pub fn new(root_dir: PathBuf) -> Dotfiles {
    let entries = read_entries(root_dir.as_path());

    Dotfiles {
      _root_dir: root_dir,
      _entries: entries,
    }
  }

  pub fn root_dir(&self) -> &Path {
    self._root_dir.as_path()
  }

  pub fn entries(&self) -> &[Entry] {
    self._entries.as_slice()
  }
}

fn read_entries(root_dir: &Path) -> Vec<Entry> {
  let entries = util::read_toml(root_dir.join(".entries")).unwrap();

  let mut buf = Vec::new();
  read_entries_from_key(buf, "general");
  read_entries_from_key(buf, util::OS_NAME);

  buf
}

fn read_entries_from_key(entries: &mut Vec<Entry>, key: &str) {
  for (ref key, ref val) in entries.get(key).unwrap().as_table().unwrap().iter() {
    if let Some(val) = val.as_str() {
      let src = util::expand_full(&format!("{}/{}", root_dir.display(), key));

      let mut dst = util::expand_full(val);
      if Path::new(&dst).is_relative() {
        dst = util::expand_full(&format!("$HOME/{}", val));
      }

      entries.push(Entry::new(&src, &dst));
    }
  }
}
