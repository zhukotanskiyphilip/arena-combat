@echo off
cd /d c:\Claude\arena_combat
cargo run > debug\game.log 2>&1
echo Logs saved to debug\game.log
pause
