#!/bin/bash

set -euo pipefail

case `uname -s` in
  Linux)
    docker exec -it --user $USER "rust" $HOME/.cargo/bin/cargo "$@"
    ;;
  *)
    cargo "$@"
esac
