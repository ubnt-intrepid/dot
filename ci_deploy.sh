#!/bin/bash -e

fname="dot-$(uname -m)-$(uname -s | tr [:upper:] [:lower:])"

cargo build --release

mkdir -p $fname
cp target/release/dot $fname/
cp completions/* $fname/

tar zcf ${fname}.tar.gz $fname/
