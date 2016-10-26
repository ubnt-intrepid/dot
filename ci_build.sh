#!/bin/bash -e

if [[ $TRAVIS_OS_NAME == "osx"   && $ARCH == "x86_64" ]]; then
  cargo build --release --target=x86_64-apple-darwin
  cargo test  --release --target=x86_64-apple-darwin
fi

if [[ $TRAVIS_OS_NAME == "osx"   && $ARCH == "i686"   ]]; then
  cargo build --release --target=i686-apple-darwin
  cargo test --release --target=i686-apple-darwin
fi

if [[ $TRAVIS_OS_NAME == "linux" && $ARCH == "x86_64" ]]; then
  cargo build --release --target=x86_64-unknown-linux-musl
  cargo test  --release --target=x86_64-unknown-linux-musl
fi

if [[ $TRAVIS_OS_NAME == "linux" && $ARCH == "i686" ]]; then
  cargo build --release --target=i686-unknown-linux-musl
  cargo test  --release --target=i686-unknown-linux-musl
fi
