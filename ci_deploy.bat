cargo build --release

mkdir %TARGET%-%HOST%
copy target\release\%TARGET%.exe .\%TARGET%-%HOST%\dot.exe
copy completions\* .\%TARGET%-%HOST%\
