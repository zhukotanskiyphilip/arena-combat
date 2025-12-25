@echo off
cd /d "%~dp0"

REM Try to find cargo
set CARGO_CMD=cargo
if exist "%USERPROFILE%\.cargo\bin\cargo.exe" set CARGO_CMD=%USERPROFILE%\.cargo\bin\cargo.exe

echo Building...
"%CARGO_CMD%" build 2> debug\build_output.log
if %ERRORLEVEL% EQU 0 (
    echo Build OK
    echo Starting game...
    target\debug\arena_combat.exe
) else (
    echo Build failed! Errors:
    type debug\build_output.log
    echo.
    echo ===== BUILD ERROR %DATE% %TIME% ===== >> debug\build_output.log
    pause
)
