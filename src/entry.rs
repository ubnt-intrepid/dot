use crate::util;
use ansi_term;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryStatus {
    Healthy,
    LinkNotCreated,
    NotSymLink,
    WrongLinkPath,
}

#[derive(Debug, Clone)]
pub struct Entry {
    src: PathBuf,
    dst: PathBuf,
}

impl Entry {
    pub fn new(src: &str, dst: &str) -> Entry {
        Entry {
            src: util::make_pathbuf(src),
            dst: util::make_pathbuf(dst),
        }
    }

    pub fn status(&self) -> Result<EntryStatus, io::Error> {
        let status = if !self.dst.exists() {
            EntryStatus::LinkNotCreated
        } else if !util::is_symlink(&self.dst)? {
            EntryStatus::NotSymLink
        } else if self.src != self.dst.read_link()? {
            EntryStatus::WrongLinkPath
        } else {
            EntryStatus::Healthy
        };

        Ok(status)
    }

    pub fn check(&self, verbose: bool) -> Result<bool, io::Error> {
        let status = self.status()?;
        if status != EntryStatus::Healthy {
            println!(
                "{} {} ({:?})",
                ansi_term::Style::new()
                    .bold()
                    .fg(ansi_term::Colour::Red)
                    .paint("✘"),
                self.dst.display(),
                status
            );
            return Ok(false);
        }
        if verbose {
            println!(
                "{} {}\n  => {}",
                ansi_term::Style::new()
                    .bold()
                    .fg(ansi_term::Colour::Green)
                    .paint("✓"),
                self.dst.display(),
                self.src.display()
            );
        }
        Ok(true)
    }

    pub fn mklink(&self, dry_run: bool, verbose: bool) -> Result<(), io::Error> {
        if !self.src.exists() || self.status()? == EntryStatus::Healthy {
            return Ok(()); // Do nothing.
        }

        if self.dst.exists() && !util::is_symlink(&self.dst)? {
            let origpath = orig_path(&self.dst);
            println!(
                "file {} has already existed. It will be renamed to {}",
                self.dst.display(),
                origpath.display()
            );
            fs::rename(&self.dst, origpath)?;
        }

        if verbose {
            println!("{}\n  => {}", self.dst.display(), self.src.display());
        }
        util::make_link(&self.src, &self.dst, dry_run)
    }

    pub fn unlink(&self, dry_run: bool, verbose: bool) -> Result<(), io::Error> {
        if !self.dst.exists() || !util::is_symlink(&self.dst)? {
            return Ok(()); // do nothing
        }

        if verbose {
            println!("unlink {}", self.dst.display());
        }
        util::remove_link(&self.dst, dry_run)?;

        let origpath = orig_path(&self.dst);
        if origpath.exists() {
            fs::rename(origpath, &self.dst)?;
        }

        Ok(())
    }
}

fn orig_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let origpath = format!("{}.bk", path.as_ref().to_str().unwrap());
    Path::new(&origpath).to_path_buf()
}
