@echo on

IF NOT DEFINED PKGNAME set PKGNAME=dot
IF NOT DEFINED HOST set HOST=arm-linux-androideabi

cargo build --release --target=%HOST%

mkdir %PKGNAME%-%HOST%\bin
mkdir %PKGNAME%-%HOST%\etc\bash_completion.d
mkdir %PKGNAME%-%HOST%\share\zsh\site-functions
mkdir %PKGNAME%-%HOST%\share\fish\completions

copy target\%HOST%\release\%PKGNAME%.exe   .\%PKGNAME%-%HOST%\bin\%PKGNAME%.exe
copy completions\%PKGNAME%.bash-completion .\%PKGNAME%-%HOST%\etc\bash_completion.d\%PKGNAME%
copy completions\_%PKGNAME%                .\%PKGNAME%-%HOST%\share\zsh\site-functions\_%PKGNAME%
copy completions\%PKGNAME%.fish            .\%PKGNAME%-%HOST%\share\fish\completions\%PKGNAME%.fish
