@echo off
REM Build script for Windows
REM Run this on a Windows machine with Rust installed

echo Building Bas Veeg Arc for Windows...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo Build failed!
    pause
    exit /b %ERRORLEVEL%
)

echo Build successful!
echo.
echo Creating distribution folder...
if not exist "dist" mkdir dist

REM Copy and rename the executable
copy "target\release\bas-veeg-arc.exe" "dist\Bas Veeg Arc.exe"
copy "app.ico" "dist\"

echo.
echo Release build complete!
echo Executable: dist\Bas Veeg Arc.exe
echo.
echo To create an installer, install NSIS and run:
echo makensis installer.nsi
echo.
pause
