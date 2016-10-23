use std::path::{Path, PathBuf};
use entry::Entry;
use util;

pub struct Dotfiles {
  _root_dir: PathBuf,
  _entries: Vec<Entry>,
}

impl Dotfiles {
  pub fn new(root_dir: PathBuf, linkfiles: &[PathBuf]) -> Dotfiles {
    let mut entries = Vec::new();
    for linkfile in linkfiles {
      entries.extend(parse_linkfile(&linkfile, &root_dir));
    }

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


fn parse_linkfile<P: AsRef<Path>, Q: AsRef<Path>>(linkfile: P, dotdir: Q) -> Vec<Entry> {
  let parsed = util::read_toml(linkfile).unwrap();

  let mut buf = Vec::new();
  for (ref key, ref val) in parsed.iter() {
    if let Some(val) = val.as_str() {
      let src = util::expand_full(&format!("{}/{}", dotdir.as_ref().display(), key));

      let mut dst = util::expand_full(val);
      if Path::new(&dst).is_relative() {
        dst = util::expand_full(&format!("$HOME/{}", val));
      }

      buf.push(Entry::new(&src, &dst));
    }
  }

  buf
}
