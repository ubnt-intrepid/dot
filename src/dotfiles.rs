use crate::entry::Entry;
use crate::util;
use std::path::{Path, PathBuf};
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

fn new_entry(root_dir: &Path, key: &str, val: &str) -> Entry {
    let src = util::expand_full(&format!("{}/{}", root_dir.display(), key)).unwrap();

    let mut dst = util::expand_full(val).unwrap();
    if Path::new(&dst).is_relative() {
        dst = util::expand_full(&format!("$HOME/{}", val)).unwrap();
    }

    Entry::new(&src, &dst)
}

fn read_entries_from_key(
    buf: &mut Vec<Entry>,
    entries: &toml::value::Table,
    root_dir: &Path,
    key: &str,
) {
    if let Some(entries_table) = entries.get(key).and_then(|value| value.as_table()) {
        for (ref key, ref val) in entries_table.iter() {
            if let Some(val) = val.as_str() {
                buf.push(new_entry(root_dir, key, val));
            }
            if let Some(val) = val.as_array() {
                for v in val {
                    if let Some(v) = v.as_str() {
                        buf.push(new_entry(root_dir, key, v));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{read_entries_from_key, Dotfiles};
    use crate::util;
    use std::path::Path;

    #[test]
    fn smoke_test() {
        let root_dir = Path::new("tests/dotfiles").to_path_buf();
        let mut dotfiles = Dotfiles::new(root_dir);
        assert_eq!(Path::new("tests/dotfiles"), dotfiles.root_dir());
        dotfiles.read_entries();
    }

    #[test]
    fn do_nothing_if_given_key_is_not_exist() {
        let root_dir = Path::new("tests/dotfiles").to_path_buf();
        let entries = util::read_toml(root_dir.join(".mappings")).unwrap();

        let mut buf = Vec::new();
        read_entries_from_key(&mut buf, &entries, &root_dir, "hogehoge");
        assert_eq!(buf.len(), 0);
    }
}
