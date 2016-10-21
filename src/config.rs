use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, BufRead, BufReader};
use toml;
use regex;
use entry::Entry;
use util;

pub struct Config {
  pub repo: String,
  pub dotdir: String,
  linkfiles: Vec<String>,
}

impl Config {
  pub fn new<P: AsRef<Path>>(path: P) -> Config {
    let config = read_toml(path).unwrap();

    let repo = config.get("clone_repository").unwrap().as_str().unwrap().to_owned();
    let dotdir = config.get("dotdir").unwrap().as_str().unwrap().to_owned();

    env::set_var("clone_repository", util::expand_full(&repo));
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
      repo: repo,
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
  let file = File::open(linkfile.as_ref()).unwrap();
  let file = BufReader::new(file);

  let re = regex::Regex::new(r"^\s*#.*$|^\s*$").unwrap();

  let mut buf = Vec::new();
  for line in file.lines() {
    let line = re.replace(&line.unwrap(), "");
    if line == "" {
      continue;
    }
    let token: Vec<_> = line.split(",").map(|s| s.trim().to_owned()).collect();

    let src = util::expand_full(&dotdir.as_ref().join(&token[0]).to_str().unwrap());

    let mut dst = util::expand_full(&token[1]);
    if Path::new(&dst).is_relative() {
      dst = util::expand_full(&format!("$HOME/{}", dst));
    }

    buf.push(Entry {
      src: Path::new(&src).to_path_buf(),
      dst: Path::new(&dst).to_path_buf(),
    });
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
