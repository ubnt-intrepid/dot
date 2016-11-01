use std::env;
use std::io::{self, Read};
use std::fs::{self, File};
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use shellexpand::{self, LookupError};
use toml;


#[allow(dead_code)]
pub fn wait_exec(cmd: &str,
                 args: &[&str],
                 curr_dir: Option<&Path>,
                 dry_run: bool)
                 -> Result<i32, io::Error> {
  if dry_run {
    println!("{} {:?} (@ {:?})", cmd, args, curr_dir);
    return Ok(0);
  }

  let mut command = Command::new(cmd);
  command.args(args)
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit());
  if let Some(curr_dir) = curr_dir {
    command.current_dir(curr_dir);
  }

  let mut child = try!(command.spawn());
  child.wait()
    .and_then(|st| st.code().ok_or(io::Error::new(io::ErrorKind::Other, "")))
}


pub fn expand_full(s: &str) -> Result<String, LookupError<env::VarError>> {
  shellexpand::full(s).map(|s| s.into_owned())
}


#[cfg(windows)]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<(), io::Error> {
  use std::os::windows::fs;
  if src.as_ref().is_dir() {
    fs::symlink_dir(src, dst)
  } else {
    fs::symlink_file(src, dst)
  }
}

#[cfg(not(windows))]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<(), io::Error> {
  use std::os::unix::fs::symlink;
  symlink(src, dst)
}

pub fn make_link<P, Q>(src: P, dst: Q, dry_run: bool) -> Result<(), io::Error>
  where P: AsRef<Path>,
        Q: AsRef<Path>
{
  if dry_run {
    println!("make_link({}, {})",
             src.as_ref().display(),
             dst.as_ref().display());
    Ok(())
  } else {
    try!(fs::create_dir_all(dst.as_ref().parent().unwrap()));
    symlink(src, dst)
  }
}


#[cfg(windows)]
fn unlink<P: AsRef<Path>>(dst: P) -> Result<(), io::Error> {
  if dst.as_ref().is_dir() {
    fs::remove_dir(dst)
  } else {
    fs::remove_file(dst)
  }
}

#[cfg(not(windows))]
fn unlink<P: AsRef<Path>>(dst: P) -> Result<(), io::Error> {
  fs::remove_file(dst)
}

pub fn remove_link<P: AsRef<Path>>(dst: P, dry_run: bool) -> Result<(), io::Error> {
  if dry_run {
    println!("fs::remove_file {}", dst.as_ref().display());
    Ok(())
  } else {
    unlink(dst)
  }
}


pub fn read_toml<P: AsRef<Path>>(path: P) -> Result<toml::Table, io::Error> {
  let mut file = try!(File::open(path));

  let mut buf = Vec::new();
  try!(file.read_to_end(&mut buf));

  let content = String::from_utf8_lossy(&buf[..]).into_owned();
  toml::Parser::new(&content).parse().ok_or(io::Error::new(io::ErrorKind::Other,
                                                           "failed to parse configuration file \
                                                            as TOML"))
}


#[cfg(target_os = "windows")]
pub static OS_NAME: &'static str = "windows";

#[cfg(target_os = "macos")]
pub static OS_NAME: &'static str = "darwin";

#[cfg(target_os = "linux")]
pub static OS_NAME: &'static str = "linux";

#[cfg(target_os = "android")]
pub static OS_NAME: &'static str = "linux";


// create an instance of PathBuf from string.
pub fn make_pathbuf(path: &str) -> PathBuf {
  let path = path.replace("/", &format!("{}", MAIN_SEPARATOR));
  Path::new(&path).to_path_buf()
}

pub fn is_symlink<P: AsRef<Path>>(path: P) -> Result<bool, io::Error> {
  let meta = try!(path.as_ref().symlink_metadata());
  Ok(meta.file_type().is_symlink())
}