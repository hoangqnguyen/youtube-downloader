<div align="center">

<img src="src-tauri/icons/128x128@2x.png" width="96" height="96" alt="Lumi Downloader icon" />

# Lumi Downloader

**A fast, minimal desktop app for downloading videos and playlists.**
Built with [Tauri v2](https://tauri.app/) (Rust) + [Svelte 5](https://svelte.dev/).

[![Release](https://img.shields.io/github/v/release/hoangqnguyen/lumi-downloader?style=flat-square)](https://github.com/hoangqnguyen/lumi-downloader/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey?style=flat-square)](#download)

<br/>

![App screenshot](docs/screenshot.png)

<br/>

**[⬇️ Download](https://github.com/hoangqnguyen/lumi-downloader/releases/latest)** · **[Report a bug](https://github.com/hoangqnguyen/lumi-downloader/issues)** · **[Donate ☕](#donate)**

</div>

---

## Features

- Paste a list of URLs (newline, comma, or space-separated) or a playlist URL
- Playlist URLs are automatically expanded into individual videos
- Parallel downloads with configurable concurrency (1–5 at a time)
- Real-time progress per video: speed, ETA, file size
- Retry all failed downloads with one click
- Auto-retry with configurable max attempts and delay
- Audio-only mode: extract MP3 from any video
- Resolution selector: Best / 1080p / 720p / 480p / 360p
- Customizable save folder (defaults to system Downloads)
- Light/dark theme toggle
- Auto-downloads yt-dlp, ffmpeg, and ffprobe on first launch — no manual setup needed
- Small binary (~5 MB), no telemetry, no ads

---

## Download

Head to the [Releases page](https://github.com/hoangqnguyen/lumi-downloader/releases/latest) and grab the file for your platform:

| Platform | File |
|----------|------|
| macOS (Apple Silicon + Intel) | `Lumi.Downloader_universal.dmg` |
| Windows | `Lumi.Downloader_x64_en-US.msi` or `.exe` |
| Linux | `.AppImage`, `.deb`, or `.rpm` |

> **macOS note:** The app is notarized and signed — you can open it normally. If you see a security warning anyway, right-click → Open.

---

## Build from source

### Prerequisites

| Tool | Version |
|------|---------|
| [Rust](https://rustup.rs/) | ≥ 1.77 |
| [Node.js](https://nodejs.org/) | ≥ 18 |

### Quick start

```bash
# 1. Clone
git clone https://github.com/hoangqnguyen/lumi-downloader
cd lumi-downloader

# 2. Install JS dependencies
npm install

# 3. Dev mode (yt-dlp + ffmpeg are auto-downloaded on first launch)
npm run tauri -- dev

# 4. Release build
npm run tauri -- build
```

> **Note:** You don't need to download yt-dlp, ffmpeg, or ffprobe manually. The app automatically downloads them on first launch.

### Linux — system dependencies

Building the Tauri app on Linux requires development libraries for GTK / WebKit. On Debian/Ubuntu:

```bash
sudo apt update
sudo apt install -y build-essential pkg-config cmake libglib2.0-dev libgdk-pixbuf2.0-dev \
  libwebkit2gtk-4.1-dev libsoup-3.0-dev libssl-dev squashfs-tools patchelf
```

On Fedora/RHEL:

```bash
sudo dnf install -y @development-tools pkgconfig cmake glib2-devel gdk-pixbuf2-devel \
  webkit2gtk4.1-devel libsoup3-devel openssl-devel squashfs-tools patchelf
```

### macOS — universal binary

To build a universal binary (Apple Silicon + Intel):

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm run tauri -- build --target universal-apple-darwin
```

### macOS — notarization

To distribute outside the App Store without Gatekeeper warnings:

```bash
export APPLE_ID="you@example.com"
export APPLE_PASSWORD="xxxx-xxxx-xxxx-xxxx"   # App-specific password
export APPLE_TEAM_ID="XXXXXXXXXX"
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (XXXXXXXXXX)"

npm run tauri -- build --target universal-apple-darwin
```

---

## Project structure

```
lumi-downloader/
├── src/                        # Svelte 5 frontend
│   ├── App.svelte
│   ├── app.css                 # Global styles + theme variables
│   ├── main.ts
│   └── lib/
│       ├── components/
│       │   ├── UrlInput.svelte
│       │   ├── QueuePanel.svelte
│       │   ├── QueueItem.svelte
│       │   ├── AdvancedPanel.svelte
│       │   ├── FolderPicker.svelte
│       │   └── SetupScreen.svelte
│       ├── stores/
│       │   ├── queue.svelte.ts     # Download queue state + scheduler
│       │   └── settings.svelte.ts  # Persistent settings (folder, quality, retry)
│       ├── tauri.ts                # Typed wrappers for Tauri commands
│       ├── parseUrls.ts            # URL parsing and playlist detection
│       └── types.ts
├── src-tauri/                  # Rust / Tauri backend
│   ├── src/
│   │   ├── lib.rs              # Plugin registration + AppState
│   │   ├── commands/
│   │   │   ├── download.rs     # start_download, cancel_download
│   │   │   ├── playlist.rs     # expand_playlist
│   │   │   ├── settings.rs     # pick_folder, open_folder
│   │   │   └── setup.rs        # check_binaries, setup_binaries
│   │   └── ytdlp/
│   │       ├── runner.rs       # yt-dlp sidecar spawn + stdout streaming
│   │       └── progress.rs     # Progress line parser
│   ├── binaries.rs             # Binary downloader (yt-dlp, ffmpeg, ffprobe)
│   ├── capabilities/default.json
│   └── tauri.conf.json
├── package.json
└── vite.config.ts
```

---

## Troubleshooting

**`webkit2gtk not found`** (Linux) — Install the system WebKit package for your distro (see Linux setup above).

**Gatekeeper warning on macOS** — Either notarize the build (see above), or right-click → Open on first launch.

**Downloads fail** — The app auto-downloads yt-dlp and ffmpeg on first launch. If that fails (e.g. network issues), restart the app to retry.

---

## Donate

Lumi Downloader is free and open source. If it saves you time, consider buying me a coffee ☕

[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/johnnyng)
[![GitHub Sponsors](https://img.shields.io/badge/GitHub%20Sponsors-EA4AAA?style=for-the-badge&logo=github-sponsors&logoColor=white)](https://github.com/sponsors/hoangqnguyen)

---

## License

MIT — see [LICENSE](LICENSE).
