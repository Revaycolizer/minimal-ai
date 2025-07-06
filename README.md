# 🧠 Minimal AI CLI Assistant

A fun, fuzzy-learning CLI bot written in Rust. It stores your name, answers fuzzy queries, does math, and even speaks responses!

---

## ✨ Features

- Learns anything you teach it
- Saves your name + data to JSON
- Fuzzy-matches questions
- Solves math expressions
- CSV / Markdown export & import
- Text-to-speech responses
- Colored, styled terminal UI

---

## 🚀 Getting Started

### Install dependencies

Make sure to install `espeak` for text-to-speech support on Linux/macOS:

```bash
sudo apt install espeak
```
Windows users should have PowerShell 5+ (usually pre-installed).

🛠️ Build Instructions
Build for your current platform (native build)
```bash
cargo build --release
```

The executable will be in:

Linux/macOS: ./target/release/minimal_ai

Windows: .\target\release\minimal_ai.exe

Cross-compiling
Windows from Linux/macOS
Add Windows target:
```bash
rustup target add x86_64-pc-windows-gnu
```

Build for Windows:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

Output binary:

```bash
./target/x86_64-pc-windows-gnu/release/minimal_ai.exe
```

macOS from Linux/Windows (requires macOS SDK & tooling)
Add macOS target:

```bash
rustup target add x86_64-apple-darwin
```
Build for macOS:

```bash
cargo build --release --target x86_64-apple-darwin
```
Output binary:

```bash
./target/x86_64-apple-darwin/release/minimal_ai
```
Note: Cross-compiling to macOS requires additional setup such as osxcross or an actual macOS machine.

📦 Packaging & Installation
# Linux .deb
Use cargo-deb:

```bash
cargo install cargo-deb
```

# Add metadata to Cargo.toml:
```toml
[package.metadata.deb]
maintainer = "Your Name <you@example.com>"
```
```bash
cargo build --release
cargo deb --no-build
sudo dpkg -i target/debian/minimal-ai_0.1.0_amd64.deb
sudo ln -s /usr/bin/minimal_ai /usr/local/bin/minimal-ai
```
# To make it have an executable
```bash
Linux (GNOME/KDE/Xfce)
Create a .desktop file so it appears in your application launcher.

Place the executable in a system path (e.g. /usr/bin/minimal_ai — your Debian package does this).

Create ~/.local/share/applications/minimal-ai.desktop with:

ini
Copy
Edit
[Desktop Entry]
Name=Minimal AI
Comment=Fuzzy‑learning CLI assistant
Exec=minimal_ai
Icon=utilities-terminal
Terminal=true
Type=Application
Categories=Utility;
Make it executable:
```
```bash
chmod +x ~/.local/share/applications/minimal-ai.desktop
```
To uninstall
```bash
sudo dpkg -r minimal-ai
```

Remove Config Files
```bash
sudo dpkg --purge minimal-ai
```
# Windows .exe installer
Use Inno Setup:

Write an installer script installer.iss:

```ini

[Setup]
AppName=Minimal AI
AppVersion=0.1
DefaultDirName={pf}\Minimal AI
DefaultGroupName=Minimal AI
OutputBaseFilename=MinimalAIInstaller
Compression=lzma
SolidCompression=yes

[Files]
Source: "target\release\minimal_ai.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Minimal AI"; Filename: "{app}\minimal_ai.exe"
```
Compile the script with Inno Setup Compiler to generate installer .exe.

macOS .app and .dmg
Use cargo-bundle:

```bash

cargo install cargo-bundle
cargo build --release
cargo bundle --release
```
This produces .app and optionally .dmg installers.

💬 Usage
Run the app, type teaching mode to teach new knowledge.

Ask questions or input math expressions.

Use commands like show, reset, export_csv, etc.

# 🗣️ Voice Support
Ensure espeak (Linux/macOS) or PowerShell (Windows) is available for speech output.