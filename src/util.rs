use std::io;
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
