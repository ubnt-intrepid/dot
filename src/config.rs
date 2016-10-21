use std::collections::BTreeMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Read, BufRead, BufReader};
use toml;
use shellexpand;
use regex;


#[derive(Clone)]
pub struct Entry {
  src: PathBuf,
  dst: PathBuf,
}

impl Entry {
  pub fn source(&self) -> &Path {
    self.src.as_path()
  }
  pub fn dest(&self) -> &Path {
    self.dst.as_path()
  }
}


#[allow(dead_code)]
pub struct Config {
  pub repo: String,
  pub dotdir: String,
  pub linkfiles: BTreeMap<String, Vec<Entry>>,
}

impl Config {
  pub fn new<P: AsRef<Path>>(path: P) -> Config {
    let config = read_toml(path).unwrap();

    let repo = config.get("clone_repository").unwrap().as_str().unwrap().to_owned();
    let dotdir = config.get("dotdir").unwrap().as_str().unwrap().to_owned();

    let mut buf = BTreeMap::new();
    for linkfile in config.get("linkfiles")
      .unwrap()
      .as_slice()
      .unwrap() {
      let linkfile = shellexpand::full(linkfile.as_str().unwrap()).unwrap().into_owned();
      buf.insert(linkfile.clone(), parse_linkfile(&linkfile, &dotdir));
    }

    Config {
      repo: repo,
      dotdir: dotdir,
      linkfiles: buf,
    }
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

    let src = expand_full(&dotdir.as_ref().join(&token[0]).to_str().unwrap());

    let mut dst = expand_full(&token[1]);
    if Path::new(&dst).is_relative() {
      dst = expand_full(&format!("$HOME/{}", dst));
    }

    buf.push(Entry {
      src: fs::canonicalize(src).unwrap(),
      dst: fs::canonicalize(dst).unwrap(),
    });
  }

  buf
}

fn expand_full(s: &str) -> String {
  shellexpand::full(s).unwrap().into_owned()
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
