@echo on

set FNAME=%TARGET%-%HOST%

mkdir %FNAME%
copy target\%HOST%\release\%TARGET%.exe .\%FNAME%\%TARGET%.exe
copy completions\* .\%FNAME%\
