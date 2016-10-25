cargo build --release

setx HOST -f host-triplet -a 0,0 > NUL
set FNAME=%TARGET%-%HOST%


mkdir %FNAME%
copy target\release\%TARGET%.exe .\%FNAME%\%TARGET%.exe
copy completions\* .\%FNAME%\
