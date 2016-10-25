cargo build --release

set /p HOST=<host-triplet
set FNAME=%TARGET%-%HOST%

echo %FNAME%

mkdir %FNAME%
copy target\release\%TARGET%.exe .\%FNAME%\%TARGET%.exe
copy completions\* .\%FNAME%\
