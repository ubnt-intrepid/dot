#!/bin/bash -e

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain beta
export PATH=~/.cargo/bin:$PATH

if [[ $TRAVIS_OS_NAME == "linux" && $ARCH == "x86_64" ]]; then
  rustup target add x86_64-unknown-linux-musl
fi

if [[ $TRAVIS_OS_NAME == "linux" && $ARCH == "i686" ]]; then
  rustup target add i686-unknown-linux-musl
fi

if [[ $TRAVIS_OS_NAME == "osx" && $ARCH == "i686" ]]; then
  rustup target add i686-apple-darwin
fi
