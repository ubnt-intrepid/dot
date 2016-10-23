use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, Read};
use toml;
use entry::Entry;
use util;

pub struct Config {
  pub dotdir: String,
  linkfiles: Vec<String>,
}

impl Config {
  pub fn new() -> Config {
    let path = "~/.dotconfig.toml";
    let extracted_path = util::expand_full(path);
    let config = read_toml(&extracted_path).unwrap();

    let dotdir = config.get("dotdir").unwrap().as_str().unwrap().to_owned();
    env::set_var("dotdir", util::expand_full(&dotdir));

    let mut linkfiles = Vec::new();
    for linkfile in config.get("linkfiles")
      .unwrap()
      .as_slice()
      .unwrap() {
      let linkfile = util::expand_full(linkfile.as_str().unwrap());
      linkfiles.push(linkfile);
    }

    Config {
      dotdir: dotdir,
      linkfiles: linkfiles,
    }
  }

  pub fn read_linkfiles(&mut self) -> BTreeMap<String, Vec<Entry>> {
    let mut buf = BTreeMap::new();
    for linkfile in self.linkfiles.iter() {
      let links = parse_linkfile(&linkfile, &self.dotdir);
      buf.insert(linkfile.clone(), links);
    }
    buf
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
