#!/bin/bash -e

export PKGNAME=${PKGNAME:-dot}
export HOST=${HOST:-arm-linux-androideabi}

cargo build --release --target=$HOST

mkdir -p ${PKGNAME}-${HOST}/{bin,etc/bash_completion.d,share/zsh/site-functions,share/fish/completions}
cp target/${HOST}/release/${PKGNAME}      ${PKGNAME}-${HOST}/bin/
cp completions/${PKGNAME}.bash-completion ${PKGNAME}-${HOST}/etc/bash_completion.d/${PKGNAME}
cp completions/_${PKGNAME}                ${PKGNAME}-${HOST}/share/zsh/site-functions/
cp completions/${PKGNAME}.fish            ${PKGNAME}-${HOST}/share/fish/completions/
tar zcf ${PKGNAME}-${HOST}.tar.gz ${PKGNAME}-${HOST}/
