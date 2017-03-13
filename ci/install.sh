#!/bin/bash

script_dir="$(cd $(dirname $BASH_SOURCE); pwd)"

set -euo pipefail

main() {
  case `uname -s` in
    Linux)
      # Install Rustup toolchain
      curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$toolchain"

      # Install target
      default_target="`$HOME/.cargo/bin/rustup target list | grep default | awk '{print $1}'`"
      if ! [[ "$target" = "$default_target" ]]; then
        "$HOME/.cargo/bin/rustup" target add "$target"
      fi

      # Launch docker container for building
      docker rm -f "$container_name" || true
      docker run --name "$container_name" -d -it --privileged \
        -v "$(pwd)":$HOME/src -w $HOME/src \
        -v "$HOME/.cargo":$HOME/.cargo \
        -v "$HOME/.rustup":$HOME/.rustup \
        "$image_name"
      docker exec -it "$container_name" useradd -ms /bin/bash "$USER"

      if ! [[ "$target" = "$default_target" ]]; then
        mkdir -p $script_dir/../.cargo
        echo -e "[build]\ntarget = \"$target\"" | tee $script_dir/../.cargo/config
        case $target in
          arm-linux-androideabi|i686-linux-android)
            echo -e "\n[target.$target]\nlinker = \"$target-gcc\"" | tee -a $script_dir/../.cargo/config ;;
        esac
      fi
      ;;

    Darwin)
      curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$toolchain" --default-host "$target"
      ;;
  esac
}

#= PARAMETERS ===================================
target="${1:-x86_64-unknown-linux-gnu}"
toolchain="${2:-stable}"
container_name="${3:-rust}"
api="${API:-24}"

case $target in
  *arm-linux-androideabi*|*i686-linux-android*)
    image_name="ubntintrepid/${target}:api${api}"
    if [[ "$target" = "arm-linux-androideabi" ]]; then arch=arm; fi
    if [[ "$target" = "i686-linux-android"    ]]; then arch=x86; fi
    if [[ `docker images -q "$image_name" | wc -l` = 0 ]]; then
      docker build -t "$image_name" \
        --build-arg ARCH="$arch" \
        --build-arg API="$api" \
        "$script_dir/rust-android-builder"
    fi
    ;;
  *-apple-darwin)
    ;;
  *)
    image_name="japaric/${target}:latest"
    docker pull "$image_name"
    ;;
esac

main
