#!/bin/bash

set -euo pipefail

script_dir="$(cd $(dirname $BASH_SOURCE); pwd)"
pkgname="$1"

rm -rf ./"${pkgname}"
$script_dir/cargo.sh install --root ./"${pkgname}"

cd "${pkgname}"
tar -zcf ../"${pkgname}.tar.gz" ./*
