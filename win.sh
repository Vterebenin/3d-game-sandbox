#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
mkdir -p /mnt/c/temp/game_example/ &&
cp target/x86_64-pc-windows-gnu/debug/game_example.exe /mnt/c/temp/game_example/game_example.exe &&
cp -r assets/ /mnt/c/temp/game_example/ &&
cd /mnt/c/temp/game_example/ &&
exec ./game_example.exe "$@"

