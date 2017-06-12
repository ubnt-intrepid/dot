use std::path::{Path, PathBuf};
use entry::Entry;
use util;
use toml;


pub struct Dotfiles {
  _root_dir: PathBuf,
  _entries: Vec<Entry>,
}

impl Dotfiles {
  pub fn new(root_dir: PathBuf) -> Dotfiles {
    Dotfiles {
      _root_dir: root_dir,
      _entries: Vec::new(),
    }
  }

  pub fn read_entries(&mut self) {
    self._entries = read_entries(self._root_dir.as_path());
  }

  pub fn root_dir(&self) -> &Path {
    self._root_dir.as_path()
  }

  pub fn entries(&self) -> &[Entry] {
    self._entries.as_slice()
  }
}

fn read_entries(root_dir: &Path) -> Vec<Entry> {
  let ref entries = util::read_toml(root_dir.join(".mappings")).unwrap();

  let mut buf = Vec::new();
  read_entries_from_key(&mut buf, entries, root_dir, "general");
  read_entries_from_key(&mut buf, entries, root_dir, util::OS_NAME);

  buf
}

fn read_entries_from_key(buf: &mut Vec<Entry>,
                         entries: &toml::value::Table,
                         root_dir: &Path,
                         key: &str) {
  if let Some(entries_table) = entries.get(key).and_then(|value| value.as_table()) {
    for (ref key, ref val) in entries_table.iter() {
      if let Some(val) = val.as_str() {
        let src = util::expand_full(&format!("{}/{}", root_dir.display(), key)).unwrap();

        let mut dst = util::expand_full(val).unwrap();
        if Path::new(&dst).is_relative() {
          dst = util::expand_full(&format!("$HOME/{}", val)).unwrap();
        }

        buf.push(Entry::new(&src, &dst));
      }
    }
  }
}
