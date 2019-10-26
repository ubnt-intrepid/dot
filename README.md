# `dot`
[![Build Status](https://travis-ci.org/ubnt-intrepid/dot.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/dot)
[![Build status](https://ci.appveyor.com/api/projects/status/mleixxiv2p6saqpw/branch/master?svg=true)](https://ci.appveyor.com/project/ubnt-intrepid/dot/branch/master)

`dot` is a command-line tool for managing dotfiles, written in Rust.

## Overview
`dot` provides a way to organize configuration files in your home directory.

## Installation
Precompiled binaries are on our [GitHub releases page](https://github.com/ubnt-intrepid/dot/releases/latest).
If you want to use the development version, try `cargo install` to build from source:

```shell-session
$ cargo install --git https://github.com/ubnt-intrepid/dot.git
```

## Example Usage
Clone your dotfiles repository from github and then create home directory symlinks:  
```sh
$ dot init ubnt-intrepid/dotfiles
```

Check if all of the links exist and are correct:
```sh
$ dot check
```

`<pattern>` determines the remote repository's URL of dotfiles.

Pattern types:

* `(http|https|ssh|git)://[username@]github.com[:port]/path-to-repo.git` – URL of dotfiles repository
* `git@github.com:path-to-repo.git` – SCP-like path
* `username/dotfiles` – GitHub user and repository
* `username` – GitHub user only (repository `dotfiles`, e.g.: `https://github.com/myuser/dotfiles`)

By default, the repository will be cloned locally to `$HOME/.dotfiles`. This can be overridden with `$DOT_DIR`.

For more information, run `dot help`.

## Configuration
`$DOT_DIR/.mappings` where the symlinks are defined in [TOML](https://github.com/toml-lang/toml). For example:

```toml
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

Use `[general]` for symlinks on all platforms. `[windows]`, `[linux]`, `[macos]` for symlinks on specific platforms.

See [my dotfiles](https://github.com/ubnt-intrepid/dotfiles) for a real example.

## License
`dot` is distributed under the MIT license.
See [LICENSE](LICENSE) for details.

## Similar Projects
- [ssh0/dot](https://github.com/ssh0/dot)  
  written in shell script
- [rhysd/dotfiles](https://github.com/rhysd/dotfiles)  
  written in Golang
