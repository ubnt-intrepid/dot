#!/bin/bash -e

# Usage:
# DOTURL=https://github.com/ubnt-intrepid/.dotfiles.git [PREFIX=$HOME/.local] ./bootstrap.sh

# Repository URL of your dotfiles.
DOT_URL=${DOT_URL:-"https://github.com/ubnt-intrepid/.dotfiles.git"}

#
DOT_DIR=${DOT_DIR:-"$HOME/.dotfiles"}

# installation directory of `dot`
PREFIX=${PREFIX:-"$HOME/.local"}


# --- export as environment variables
export DOT_DIR


# --- download `dot.rs` from GitHub Releases and install
case `uname -s | tr '[A-Z]' '[a-z]'` in
  *mingw* | *msys*)
    DOTRS_SUFFIX="`uname -m`-windows-msvc"
    ;;
  *darwin*)
    DOTRS_SUFFIX="`uname -m`-apple-darwin"
    ;;
  *linux*)
    DOTRS_SUFFIX="`uname -m`-unknown-linux-musl"
    ;;
  *android*)
    # TODO: support for other architectures
    DOTRS_SUFFIX="arm-linux-androideabi"
    ;;
  *)
    echo "[fatal] cannot recognize the platform."
    exit 1
esac

DOTRS_URL="`curl -s https://api.github.com/repos/ubnt-intrepid/dot.rs/releases | grep browser_download_url | cut -d '"' -f 4 | grep "$DOTRS_SUFFIX" | head -n 1`"
echo "$DOTRS_URL"

mkdir -p "${PREFIX}/bin"
curl -sL "${DOTRS_URL}" | tar xz -C "$PREFIX/bin/" --strip=1 './dot'

export PATH="$PREFIX/bin:$PATH"

# --- clone your dotfiles into home directory, and make links.
[[ -d "$DOT_DIR" ]] || git clone "$DOT_URL" "$DOT_DIR"
dot link --verbose
