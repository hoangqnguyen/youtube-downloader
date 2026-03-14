# YouTube Downloader

A fast, minimal desktop app for downloading YouTube videos and playlists. Built with [Tauri v2](https://tauri.app/) (Rust backend) and [Svelte 5](https://svelte.dev/) (frontend).

![App screenshot](docs/screenshot.png)

## Features

- Paste a list of URLs (newline, comma, or space-separated) or a playlist URL
- Playlist URLs are automatically expanded into individual videos
- Parallel downloads with configurable concurrency
- Real-time progress per video (speed, ETA, file size)
- Customizable save folder (defaults to system Downloads)
- Advanced options: audio-only (MP3), resolution selector (Best / 1080p / 720p / 480p / 360p)
- Dark orange theme, small binary (~5 MB)

---

## Prerequisites

All platforms require the same base tools, plus platform-specific steps below.

| Tool | Version | Install |
|------|---------|---------|
| [Rust](https://rustup.rs/) | ≥ 1.77 | `rustup` |
| [Node.js](https://nodejs.org/) | ≥ 18 | see below |
| [yt-dlp](https://github.com/yt-dlp/yt-dlp) | latest | see below |
| ffmpeg | latest | see below |

---

## Platform Setup

### Windows

1. **Install Rust (MSVC toolchain)**

   ```powershell
   winget install Rustlang.Rustup
   # After install, open a NEW terminal so PATH updates
   rustup default stable-x86_64-pc-windows-msvc
   ```

2. **Install Visual Studio Build Tools** (required by the MSVC linker)

   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools --override "--quiet --add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows11SDK.26100"
   ```

   > If you already have Visual Studio 2019/2022 installed, the C++ workload is sufficient.

3. **Install Node.js**

   ```powershell
   winget install OpenJS.NodeJS.LTS
   ```

4. **Install yt-dlp and ffmpeg**

   ```powershell
   winget install yt-dlp.yt-dlp
   winget install yt-dlp.FFmpeg
   ```

5. **Copy yt-dlp as Tauri sidecar**

   The sidecar binary must be named with the Rust target triple. From the project root in **PowerShell**:

   ```powershell
   $src = (Get-Command yt-dlp).Source
   Copy-Item $src "src-tauri\binaries\yt-dlp-x86_64-pc-windows-msvc.exe"
   ```

   Or find it manually — it's typically at:
   ```
   C:\Users\<you>\AppData\Local\Microsoft\WinGet\Packages\yt-dlp.yt-dlp_*\yt-dlp.exe
   ```

6. **Configure the MSVC linker** (only needed if building from Git Bash / WSL)

   If `cargo build` fails with a linker error, create `src-tauri/.cargo/config.toml`:

   ```toml
   [target.x86_64-pc-windows-msvc]
   linker = "C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Tools\\MSVC\\14.xx.xxxxx\\bin\\Hostx64\\x64\\link.exe"
   rustflags = [
     "-Lnative=C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\10.0.xxxxx.0\\um\\x64",
     "-Lnative=C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\10.0.xxxxx.0\\ucrt\\x64",
     "-Lnative=C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Tools\\MSVC\\14.xx.xxxxx\\lib\\x64",
   ]
   ```

   Replace `14.xx.xxxxx` and `10.0.xxxxx.0` with the actual version folders found in those paths.

   > This is only needed when building from Git Bash, MSYS2, or WSL where the wrong `link.exe` may be on PATH. Building from PowerShell or the VS Developer Command Prompt does not need this.

---

### macOS

1. **Install Rust**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

   For Apple Silicon, the default target is `aarch64-apple-darwin`. To also build for Intel:

   ```bash
   rustup target add x86_64-apple-darwin
   ```

   To build a universal binary (both architectures):

   ```bash
   rustup target add aarch64-apple-darwin x86_64-apple-darwin
   ```

2. **Install Xcode Command Line Tools** (provides the macOS linker)

   ```bash
   xcode-select --install
   ```

3. **Install Node.js**

   ```bash
   brew install node
   # or: https://nodejs.org/
   ```

4. **Install yt-dlp and ffmpeg**

   ```bash
   brew install yt-dlp ffmpeg
   ```

5. **Copy yt-dlp as Tauri sidecar**

   ```bash
   # Apple Silicon
   cp $(which yt-dlp) src-tauri/binaries/yt-dlp-aarch64-apple-darwin

   # Intel
   cp $(which yt-dlp) src-tauri/binaries/yt-dlp-x86_64-apple-darwin

   # Both (for universal build)
   cp $(which yt-dlp) src-tauri/binaries/yt-dlp-aarch64-apple-darwin
   cp $(which yt-dlp) src-tauri/binaries/yt-dlp-x86_64-apple-darwin
   ```

   Make them executable:

   ```bash
   chmod +x src-tauri/binaries/yt-dlp-*
   ```

---

### Linux

1. **Install Rust**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install system dependencies** (required by Tauri / WebKitGTK)

   **Ubuntu / Debian:**
   ```bash
   sudo apt update
   sudo apt install -y \
     build-essential \
     libwebkit2gtk-4.1-dev \
     libssl-dev \
     libgtk-3-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev \
     patchelf
   ```

   **Fedora / RHEL:**
   ```bash
   sudo dnf install -y \
     webkit2gtk4.1-devel \
     openssl-devel \
     gtk3-devel \
     libappindicator-gtk3-devel \
     librsvg2-devel
   ```

   **Arch:**
   ```bash
   sudo pacman -S --needed \
     webkit2gtk-4.1 \
     base-devel \
     openssl \
     appmenu-gtk-module \
     libappindicator-gtk3 \
     librsvg
   ```

3. **Install Node.js**

   ```bash
   # Ubuntu/Debian
   curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
   sudo apt install -y nodejs

   # Or via nvm: https://github.com/nvm-sh/nvm
   ```

4. **Install yt-dlp and ffmpeg**

   ```bash
   # via pip (recommended for latest version)
   pip install -U yt-dlp

   # ffmpeg
   sudo apt install ffmpeg        # Ubuntu/Debian
   sudo dnf install ffmpeg        # Fedora
   sudo pacman -S ffmpeg          # Arch
   ```

5. **Copy yt-dlp as Tauri sidecar**

   ```bash
   cp $(which yt-dlp) src-tauri/binaries/yt-dlp-x86_64-unknown-linux-gnu
   chmod +x src-tauri/binaries/yt-dlp-x86_64-unknown-linux-gnu
   ```

---

## Build

### 1. Install npm dependencies

```bash
npm install
```

### 2. Dev mode (hot reload)

```bash
npm run tauri -- dev
```

### 3. Release build

```bash
npm run tauri -- build
```

Output locations:

| Platform | Format | Path |
|----------|--------|------|
| Windows | NSIS installer | `src-tauri/target/release/bundle/nsis/*.exe` |
| Windows | MSI | `src-tauri/target/release/bundle/msi/*.msi` |
| macOS | .app + .dmg | `src-tauri/target/release/bundle/macos/` |
| macOS (universal) | see below | — |
| Linux | .deb | `src-tauri/target/release/bundle/deb/` |
| Linux | .rpm | `src-tauri/target/release/bundle/rpm/` |
| Linux | AppImage | `src-tauri/target/release/bundle/appimage/` |

**macOS universal binary** (runs natively on both Apple Silicon and Intel):

```bash
npm run tauri -- build --target universal-apple-darwin
```

---

## Sidecar binary naming reference

Tauri requires the bundled binary to be named `yt-dlp-{target-triple}`. The triple must match the output of `rustc -vV | grep host`.

| Platform | Triple |
|----------|--------|
| Windows (MSVC) | `yt-dlp-x86_64-pc-windows-msvc.exe` |
| macOS Apple Silicon | `yt-dlp-aarch64-apple-darwin` |
| macOS Intel | `yt-dlp-x86_64-apple-darwin` |
| Linux x64 | `yt-dlp-x86_64-unknown-linux-gnu` |

To find your exact triple:

```bash
rustc -vV
# look for the "host:" line
```

---

## Project structure

```
youtube-downloader/
├── src/                        # Svelte 5 frontend
│   ├── App.svelte
│   ├── app.css                 # Global styles + orange theme variables
│   ├── main.ts
│   └── lib/
│       ├── components/
│       │   ├── UrlInput.svelte
│       │   ├── QueuePanel.svelte
│       │   ├── QueueItem.svelte
│       │   ├── AdvancedPanel.svelte
│       │   └── FolderPicker.svelte
│       ├── stores/
│       │   ├── queue.svelte.ts     # Download queue state + scheduler
│       │   └── settings.svelte.ts  # Persistent settings (folder, quality)
│       ├── tauri.ts                # Typed wrappers for Tauri commands
│       ├── parseUrls.ts            # URL parsing and playlist detection
│       └── types.ts
├── src-tauri/                  # Rust / Tauri backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs              # Plugin registration + AppState
│   │   ├── commands/
│   │   │   ├── download.rs     # start_download, cancel_download
│   │   │   ├── playlist.rs     # expand_playlist
│   │   │   └── settings.rs     # pick_folder, open_folder, get_default_download_dir
│   │   └── ytdlp/
│   │       ├── runner.rs       # Sidecar spawn + stdout streaming
│   │       └── progress.rs     # yt-dlp progress line parser
│   ├── binaries/               # yt-dlp sidecar binaries (gitignored)
│   ├── capabilities/
│   │   └── default.json        # Tauri v2 permission declarations
│   └── tauri.conf.json
├── package.json
├── vite.config.ts
└── svelte.config.js
```

---

## Troubleshooting

**`error: could not find yt-dlp sidecar`**
The binary is missing or misnamed. Double-check the file exists in `src-tauri/binaries/` with the exact triple suffix matching your platform (see table above).

**`LINK : fatal error LNK1181: cannot open input file 'dbghelp.lib'`** (Windows)
The MSVC linker can't find the Windows SDK libraries. Configure `src-tauri/.cargo/config.toml` with the correct lib paths (see Windows setup step 6).

**`link: extra operand` / wrong linker used** (Windows + Git Bash)
Git Bash ships its own `link` that shadows MSVC's. Configure the explicit linker path as described in Windows setup step 6.

**`webkit2gtk not found`** (Linux)
Install the system WebKit dependencies listed in the Linux setup section for your distro.

**`Permission denied` on sidecar** (macOS / Linux)
Run `chmod +x src-tauri/binaries/yt-dlp-*` to make the binary executable.

**Downloads fail with ffmpeg error**
yt-dlp needs ffmpeg to merge separate video+audio streams (required for 1080p+ quality). Make sure ffmpeg is installed and on PATH, or install it to the same directory as yt-dlp.
