# Pomodoro Study Mobile

Android mobile build of Pomodoro Study. This repository contains the mobile-ready Tauri 2 + Svelte 5 + Rust + SQLite project, including the generated Android project files under `src-tauri/gen/android`.

The mobile version intentionally tracks the desktop version number. Current version: `0.2.0`.

## Quick Start

```bash
corepack pnpm install
corepack pnpm check
corepack pnpm test
cargo test --manifest-path src-tauri/Cargo.toml
```

Rust, Android SDK/NDK, JDK 17, and pnpm are required for Android packaging.

## Android Build

```bash
corepack pnpm tauri android build --target aarch64 --apk --ci
```

If building the Android Rust library manually, the release build must include Tauri's production custom protocol feature:

```bash
corepack pnpm build
cargo build --target aarch64-linux-android --release --features tauri/custom-protocol
```

The APK package id is `study.pomodoro.app`.

## Data Location

Android stores app data in the app sandbox for `study.pomodoro.app`.

## Documentation

- [docs/ANDROID_MOBILE.md](docs/ANDROID_MOBILE.md)
- [docs/superpowers/specs/2026-06-18-android-mobile-design.md](docs/superpowers/specs/2026-06-18-android-mobile-design.md)
- [docs/AGENT_GUIDE.md](docs/AGENT_GUIDE.md)
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [docs/DATA_MODEL.md](docs/DATA_MODEL.md)
- [docs/THEMING.md](docs/THEMING.md)
- [docs/IPC_API.md](docs/IPC_API.md)
