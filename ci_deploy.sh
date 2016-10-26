#!/bin/bash -e

if [[ $TRAVIS_OS_NAME == "osx"   && $ARCH == "x86_64" ]]; then export TARGET=x86_64-apple-darwin;       fi
if [[ $TRAVIS_OS_NAME == "osx"   && $ARCH == "i686"   ]]; then export TARGET=i686-apple-darwin;         fi
if [[ $TRAVIS_OS_NAME == "linux" && $ARCH == "x86_64" ]]; then export TARGET=x86_64-unknown-linux-musl; fi
if [[ $TRAVIS_OS_NAME == "linux" && $ARCH == "i686"   ]]; then export TARGET=i686-unknown-linux-musl;   fi
 
fname="dot-$TARGET"

mkdir -p $fname
cp target/release/dot $fname/
cp completions/* $fname/

tar zcf ${fname}.tar.gz $fname/
