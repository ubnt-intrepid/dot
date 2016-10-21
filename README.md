# dot.rs
Yet another management tool for dotfiles

## Features
* written in Rust
* single binary
* partially compatible with https://github.com/ssh0/dot

## Installation

```shell-session
$ git clone https://github.com/ubnt-intrepid/dot.rs.git
$ cd dot.rs
$ cargo install
```

## Usage

1. write configuration into `~/.dotconfig.toml`

```toml
clone_repository = "https://github.com/ubnt-intrepid/.dotfiles.git"

dotdir = "$HOME/.dotfiles"

linkfiles = [
  "$dotdir/dotlink",
  "$dotdir/dotlink.linux"
]
```

2. type following command:

```shell-session
$ dot init
```

## License
`dot.rs` is released under the MIT license. See [LICENSE](LICENSE) for details.
