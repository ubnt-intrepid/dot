# `dot`
[![Build Status](https://travis-ci.org/ubnt-intrepid/dot.rs.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/dot)
[![Build status](https://ci.appveyor.com/api/projects/status/mleixxiv2p6saqpw/branch/master?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/dot/branch/master)

`dot` is a command-line tool for management dotfiles, written in Rust.

## Overview
`dot` provides a way to organize your configuration files located at home directory.

## Installation
Precompiled binaries are located at [GitHub releases page](https://github.com/ubnt-intrepid/dot/releases/latest).
If you want to use development version, try `cargo install` to build from source, as follows:

```shell-session
$ cargo install --git https://github.com/ubnt-intrepid/dot.rs.git
```

## Example Usage
* Clones your dotfiles repository from remote and then creates links into your home directory:  
  ```sh
  $ dot init ubnt-intrepid/dotfiles
  ```

* Check if all of links are correctly existed:
  ```sh
  $ dot check
  ```

`<pattern>` is set the string to determine remote repository's URL of dotfiles.
Available patterns are as follows:

* `(http|https|ssh|git)://[username@]github.com[:port]/path-to-repo.git`  
  URL of dotfiles repository
* `git@github.com:path-to-repo.git`  
  SCP-like path
* `username/dotfiles`  
  GitHub user and the name of repository
* `username`
  Only GitHub user (the name of repository is assumed to be `dotfiles`)

By default, the location of dotfiles repository is `$HOME/.dotfiles`.
The location can be specified by using environment variable `$DOT_DIR`.

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

See also [my dotfiles](https://github.com/ubnt-intrepid/dotfiles) for details.

## License
`dot` is distributed under the MIT license.
See [LICENSE](LICENSE) for details.

## Similar Projects
- [ssh0/dot](https://github.com/ssh0/dot)  
  written in shell script
- [rhysd/dotfiles](https://github.com/rhysd/dotfiles)  
  written in Golang
