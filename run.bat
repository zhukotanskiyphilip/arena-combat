@echo off
cd /d "%~dp0"
echo Building...
cargo build
if %ERRORLEVEL% EQU 0 (
    echo Starting game...
    target\debug\arena_combat.exe
) else (
    echo Build failed!
    pause
)
