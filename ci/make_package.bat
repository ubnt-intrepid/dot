@echo on

IF NOT DEFINED PKGNAME set PKGNAME=dot
IF NOT DEFINED HOST set HOST=arm-linux-androideabi

cargo build --release --target=%HOST%

if     exist "%PKGNAME%-%HOST%\" del   %PKGNAME%-%HOST%
if not exist "%PKGNAME%-%HOST%\" mkdir %PKGNAME%-%HOST%

copy target\%HOST%\release\%PKGNAME%.exe   "%PKGNAME%-%HOST%\"
