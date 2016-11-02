#!/bin/bash -e

export PKGNAME=${PKGNAME:-dot}
export HOST=${HOST:-arm-linux-androideabi}

cargo build --release --target=$HOST

rm -rf ${PKGNAME}-${HOST}
mkdir -p ${PKGNAME}-${HOST}
cp target/${HOST}/release/${PKGNAME} ${PKGNAME}-${HOST}/
cd ${PKGNAME}-${HOST}/
tar -zcf ../${PKGNAME}-${HOST}.tar.gz ./*
