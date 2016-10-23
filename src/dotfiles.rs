use std::env;
use std::fs::File;
use std::path::{self, Path, PathBuf};
use std::io::{self, Read};
use toml;
use entry::Entry;
use util;

pub struct Dotfiles {
  _root_dir: PathBuf,
  _entries: Vec<Entry>,
}

impl Dotfiles {
  pub fn new() -> Dotfiles {
    // load configuration file.
    let path = "~/.dotconfig.toml";
    let extracted_path = util::expand_full(path);
    let config = read_toml(&extracted_path).unwrap();

    //
    let dotdir = config.get("dotdir").unwrap().as_str().unwrap().to_owned();
    let dotdir = dotdir.replace("/", &format!("{}", path::MAIN_SEPARATOR));
    let dotdir = util::expand_full(&dotdir);
    let dotdir = Path::new(&dotdir).to_path_buf();

    env::set_var("dotdir", dotdir.as_os_str());

    let mut entries = Vec::new();
    for linkfile in config.get("linkfiles")
      .unwrap()
      .as_slice()
      .unwrap() {
      let linkfile = util::expand_full(linkfile.as_str().unwrap());
      entries.extend(parse_linkfile(&linkfile, &dotdir));
    }

    Dotfiles {
      _root_dir: dotdir,
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
  let parsed = read_toml(linkfile).unwrap();

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

fn read_toml<P: AsRef<Path>>(path: P) -> Result<toml::Table, io::Error> {
  let mut file = try!(File::open(path));

  let mut buf = Vec::new();
  try!(file.read_to_end(&mut buf));

  let content = String::from_utf8_lossy(&buf[..]).into_owned();
  toml::Parser::new(&content).parse().ok_or(io::Error::new(io::ErrorKind::Other,
                                                           "failed to parse configuration file \
                                                            as TOML"))
}
