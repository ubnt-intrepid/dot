param(
  [parameter(mandatory)]
  [string] $msystem,

  [parameter(mandatory)]
  [string] $root
)

switch ($msystem) {
  "MINGW32" { $target = "mingw-w64-i686" }
  "MINGW64" { $target = "mingw-w64-x86_64" }
}

function invoke-pacman {
  $bash = Join-Path -path $root -childpath "/usr/bin/bash.exe"
  & $bash -l -c ("pacman --noconfirm " + $args -join " ")
}

invoke-pacman -Syuu
invoke-pacman -Syuu
invoke-pacman -S make `
                 "$($target)-toolchain" `
                 "$($target)-curl" `
                 "$($target)-diffutils" `
                 "$($target)-perl"