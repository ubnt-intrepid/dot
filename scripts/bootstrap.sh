#!/bin/bash -e

# Usage:
# DOTURL=https://github.com/ubnt-intrepid/.dotfiles.git [PREFIX=$HOME/.local] ./bootstrap.sh

# Repository URL of your dotfiles.
DOT_URL=${DOT_URL:-"https://github.com/ubnt-intrepid/.dotfiles.git"}

# 
DOT_DIR=${DOT_DIR:-"$HOME/.dotfiles"}

# installation directory of `dot`
PREFIX=${PREFIX:-"$HOME/.local"}


# --- install dot.rs
case `uname -s | tr '[A-Z]' '[a-z]'` in
  *mingw* | *msys*)
    SUFFIX="`uname -m`-windows-msvc"
    ;;
  *darwin*)
    SUFFIX="`uname -m`-apple-darwin"
    ;;
  *linux*)
    SUFFIX="`uname -m`-unknown-linux-musl"
    ;;
  *android*)
    # TODO: support for other architectures
    SUFFIX="arm-linux-androideabi"
    ;;
  *)
    echo "[fatal] cannot recognize the platform."
    exit 1
esac
mkdir -p "$PREFIX/bin" && cd "$PREFIX/bin"
curl -L "https://github.com/ubnt-intrepid/dot.rs/releases/download/latest/dot-${SUFFIX}.tar.gz" | tar xzf -


export PATH="$PREFIX/bin:$PATH"

# --- clone your dotfiles into home directory
export DOT_DIR

git clone "$DOT_URL" "$DOT_DIR"

dot link --verbose
