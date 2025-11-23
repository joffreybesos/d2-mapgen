@echo off
echo Building d2-map as 32-bit executable...
echo.

REM Build in release mode (32-bit automatically via .cargo/config.toml)
cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo Build failed!
    exit /b %errorlevel%
)

echo.
echo ========================================
echo Build successful!
echo ========================================
echo.
echo Executable: target\i686-pc-windows-msvc\release\d2-map.exe
echo Architecture: 32-bit (PE32 Intel 80386)
echo.
echo Example usage:
echo   target\i686-pc-windows-msvc\release\d2-map.exe "E:/Dev/d2-mapserver/dist2/game" -s 12345 -d 2 -m 1 -vv
echo.


target\i686-pc-windows-msvc\release\d2-map.exe "E:/Dev/d2-mapserver/dist2/game" -s 12345 -d 2 -m 1 -vv