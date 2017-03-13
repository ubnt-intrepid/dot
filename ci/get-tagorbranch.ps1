$name = (git describe --tags --exact-match 2>$null)
if ($name -eq $null) {
  $name = (git symbolic-ref -q --short HEAD 2>$null)
}
if ($name -eq $null) {
  $name = (git rev-parse --short HEAD 2>$null)
}
if ($name -eq $null) {
  $name = "UNKNOWN"
}
Write-Output $name
exit 0
