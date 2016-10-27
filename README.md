# dot.rs

| Travis | Appveyor | Wercker |
|:------:|:--------:|:-------:|
| [![Build Status](https://travis-ci.org/ubnt-intrepid/dot.rs.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/dot.rs)  | [![Build status](https://ci.appveyor.com/api/projects/status/bh02mka0to2e6wsi/branch/master?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/dot-rs/branch/master) | [![wercker status](https://app.wercker.com/status/2c423ff1fdddb547df42c1963c525aba/s/master "wercker status")](https://app.wercker.com/project/byKey/2c423ff1fdddb547df42c1963c525aba) |

`dot.rs` is a tiny CLI tool for management of dotfiles, written in Rust.

This project is based on [ssh0](https://github.com/ssh0)'s [dot](https://github.com/ssh0/dot), and
inspired by [rhysd](https://github.com/rhysd)'s [dotfiles](https://github.com/rhysd/dotfiles).

## Getting started
```shell-session
$ git clone https://github.com/ubnt-intrepid/dot.rs.git
$ cd dot.rs
$ cargo install
```

1. Clone your dotfiles repository (`dot clone`).
1. Create all of the link (`dot link`)

```shell-session
$ export DOT_DIR=/home/example/.dotfiles
$ dot clone https://github.com/example/dotfiles.git $DOT_DIR
$ dot link
```

## Commands
* `link [-v | --verbose] [-n | --dry-run]`  
  Create all of managed links into your home directory.
* `clean [-v | --verbose] [-n | --dry-run]`  
  Remove all of managed links from your home directory.
* `check [-v | --verbose]`  
  Check if all of links are correctly existed.
* `root`  
  Show the root directory of dotfiles repository.
* `clone <url> [<dotdir>] [-n | --dry-run]`  
  Clone your dotfiles repository from remote.

If you want more information, type `dot help`.

## Configuration
See also [my dotfiles](https://github.com/ubnt-intrepid/.dotfiles).

```toml
# $DOT_DIR/.entries

[general]
gitconfig   = "~/.gitconfig"
"vim/vimrc" = "~/.vimrc"
#...

[windows]
vscode = "$APPDATA/Code/User"
powershell = "$HOME/Documents/WindowsPowerShell"
#...

[linux]
xinitrc = "~/.xinitrc"
```

## License
`dot.rs` is released under the MIT license. See [LICENSE](LICENSE) for details.
