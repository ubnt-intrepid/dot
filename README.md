# dot.rs

| Travis | Appveyor | Wercker |
|:------:|:--------:|:-------:|
| [![Build Status](https://travis-ci.org/ubnt-intrepid/dot.rs.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/dot.rs)  | [![Build status](https://ci.appveyor.com/api/projects/status/bh02mka0to2e6wsi/branch/master?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/dot-rs/branch/master) | [![wercker status](https://app.wercker.com/status/2c423ff1fdddb547df42c1963c525aba/s/master "wercker status")](https://app.wercker.com/project/byKey/2c423ff1fdddb547df42c1963c525aba) |

`dot.rs` is a tiny CLI tool for management of dotfiles, written in Rust.

This project is based on [ssh0](https://github.com/ssh0)'s [dot](https://github.com/ssh0/dot), and
inspired by [rhysd](https://github.com/rhysd)'s [dotfiles](https://github.com/rhysd/dotfiles).

## Usage
Firstly, you must clone your own dotfiles repository by using `git` command.
And then, use `dot link` to create symbolic links.

```shell-session
$ git clone https://github.com/example/dotfiles.git
$ dot link
```
By default, the location of dotfiles repository is `$HOME/.dotfiles`.
The location can be specified by using environment variable `$DOT_DIR`.

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
You must specify all of the mappings from files in dotfiles to the desired path, in `$DOT_DIR/.mappings`.
An example of `.mappings` is as follows:

```toml
# $DOT_DIR/.mappings

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

Mappings at the section `[general]` are avaialble at any environment.
On the other hand, items at the other section (`[windows]`, `[linux]`) only apply specified platform.
The value of environment variables in each items are extracted.

See also [my dotfiles](https://github.com/ubnt-intrepid/.dotfiles) for details.

## Installation
Precompiled binaries are located at [GitHub releases page](https://github.com/ubnt-intrepid/dot.rs/releases/latest).
If you want to use development version, try `cargo install` to build from source, as follows:

```shell-session
$ cargo install --git clone https://github.com/ubnt-intrepid/dot.rs.git
```

## License
`dot.rs` is released under the MIT license. See [LICENSE](LICENSE) for details.
