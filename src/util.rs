use std::io;
use std::fs;
use std::process::{Command, Stdio};
use std::path::Path;
use shellexpand;


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


pub fn expand_full(s: &str) -> String {
  shellexpand::full(s).unwrap().into_owned()
}


#[cfg(windows)]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<(), io::Error> {
  use std::os::windows::fs::symlink_file;
  symlink_file(src, dst)
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

pub fn remove_link<P: AsRef<Path>>(dst: P, dry_run: bool) -> Result<(), io::Error> {
  if dry_run {
    println!("fs::remove_file {}", dst.as_ref().display());
    Ok(())
  } else {
    fs::remove_file(dst)
  }
}
