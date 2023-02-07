#!/usr/bin/pwsh

Set-Location .\code_gen\
cargo build --release
Set-Location ..
.\target\release\code_gen.exe

Get-ChildItem .\tuples\src\gen -Filter *.rs | Foreach-Object { rustfmt $_.FullName }
