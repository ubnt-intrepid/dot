# dot.rs

[![Build Status](https://travis-ci.org/ubnt-intrepid/dot.rs.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/dot.rs)
[![Build status](https://ci.appveyor.com/api/projects/status/bh02mka0to2e6wsi/branch/master?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/dot-rs/branch/master)

`dot.rs` is a tiny CLI tool for management of dotfiles, written in Rust.

This project is based on [ssh0](https://github.com/ssh0)'s [dot](https://github.com/ssh0/dot) and
inspired by [rhysd](https://github.com/rhysd)'s [dotfiles](https://github.com/rhysd/dotfiles).

## Features
* Minimal dependency
* Available on multiple platforms: Windows, Linux and MacOSX (untested)

## Installation

```shell-session
$ git clone https://github.com/ubnt-intrepid/dot.rs.git
$ cd dot.rs
$ cargo install
```

## Getting started (Experimental)
1. Clone your dotfiles repository with `git` command.
1. Set the value of `$DOT_DIR` to the location of dotfiles repository.
1. type `dot link`

```shell-session
$ git clone --recursive https://github.com/example/dotfiles.git /home/example/.dotfiles
$ export DOT_DIR=/home/example/.dotfiles
$ dot link
```

```shell-session
$ echo 'export DOT_DIR="$HOME/.dotfiles"' >> ~/.bashrc
```

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
