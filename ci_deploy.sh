#!/bin/bash -e

fname="dot-$(cat host-triplet)"

cargo build --release

mkdir -p $fname
cp target/release/dot $fname/
cp completions/* $fname/

tar zcf ${fname}.tar.gz $fname/
