#!/bin/sh
cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/game_example.exe . &&
exec ./game_example.exe "$@"
