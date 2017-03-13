#!/bin/bash

set -euo pipefail

api_key="$1"

echo "$api_key" | cargo login
cargo package
cargo publish
