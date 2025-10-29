# Build Instructions for Bas Veeg Arc

## Building on Windows

### Prerequisites
1. Install Rust from https://rustup.rs/
2. Install NSIS (Nullsoft Scriptable Install System) from https://nsis.sourceforge.io/ (for creating installer)

### Steps

1. **Build the release version:**
   ```cmd
   build_windows.bat
   ```

   This will:
   - Compile the game in release mode with optimizations
   - Create a `dist` folder
   - Copy and rename the executable to "Bas Veeg Arc.exe"
   - Embed the app.ico icon into the executable

2. **Create the Windows installer (optional):**
   ```cmd
   makensis installer.nsi
   ```

   This will create `Bas_Veeg_Arc_Installer.exe` which:
   - Installs the game to Program Files
   - Creates Start Menu shortcuts
   - Creates a Desktop shortcut
   - Adds an uninstaller
   - Registers the application in Windows Add/Remove Programs

### Manual Build

If you prefer to build manually:

```cmd
cargo build --release
copy "target\release\bas-veeg-arc.exe" "Bas Veeg Arc.exe"
```

## Building on Linux

### Prerequisites
- Rust toolchain (rustc, cargo)

### Steps

1. **Build release version:**
   ```bash
   cargo build --release
   ```

2. **Copy and rename:**
   ```bash
   mkdir -p dist
   cp target/release/bas-veeg-arc "dist/Bas Veeg Arc"
   ```

## Cross-Compilation

### Linux → Windows

Install the Windows target and mingw:

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64  # On Debian/Ubuntu
```

Then build:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

The .exe will be in `target/x86_64-pc-windows-gnu/release/`

## Release Files

After building, the following files will be in the `dist` folder:
- `Bas Veeg Arc.exe` (Windows) or `Bas Veeg Arc` (Linux) - The game executable
- `app.ico` - Application icon

## Icon Information

The game uses `app.ico` for:
- Windows executable icon
- Desktop shortcut icon
- Start Menu shortcut icon
- Installer icon

## Version Information

The executable contains embedded metadata:
- **Product Name:** Bas Veeg Arc
- **File Description:** Bas Veeg Arc - School Fighting Game
- **Version:** 1.0.0
- **Company:** BAS VEEG ARC
- **Copyright:** © 2025
