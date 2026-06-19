# Android Mobile Guide

## Scope

Pomodoro Study targets Android 10+ through Tauri Android. The first distribution path is a signed APK for local install on the user's Android phone and tablet, followed by upload to GitHub Releases after manual verification.

This project does not target iOS, Google Play, cloud sync, or a separate native Android rewrite for the first mobile version.

## Required Tools

- Node.js and pnpm through Corepack.
- Rust toolchain.
- Android Studio.
- Android SDK Platform for Android 10+.
- Android SDK Build Tools.
- Android NDK installed through Android Studio.
- Java runtime compatible with the installed Android Gradle plugin.

Useful checks:

```powershell
corepack pnpm tauri android --help
where.exe java
where.exe adb
```

If `cargo` is not on PATH in this Codex environment, use the bundled Rust toolchain from the prior workspace:

```powershell
$env:CARGO_HOME='C:\Users\31445\Documents\Codex\2026-06-17\c-users-31445-documents-pomodoro-study\work\rust\cargo'
$env:RUSTUP_HOME='C:\Users\31445\Documents\Codex\2026-06-17\c-users-31445-documents-pomodoro-study\work\rust\rustup'
& 'C:\Users\31445\Documents\Codex\2026-06-17\c-users-31445-documents-pomodoro-study\work\rust\cargo\bin\cargo.exe' test --manifest-path src-tauri\Cargo.toml
```

## Project Commands

Install and verify the shared desktop/mobile code:

```powershell
corepack pnpm install
corepack pnpm check
corepack pnpm test
corepack pnpm lint
```

Initialize Android project files:

```powershell
corepack pnpm tauri android init
```

Run on a connected Android device during development:

```powershell
corepack pnpm tauri android dev
```

Build Android release artifacts:

```powershell
corepack pnpm tauri android build
```

The tracked Android project lives under `src-tauri/gen/android`. Generated build outputs, copied native libraries, generated Tauri Android properties, signing files, and local APK outputs stay out of git.

When manually building the Android Rust library outside the Tauri CLI, include the production custom protocol feature:

```powershell
corepack pnpm build
cargo build --target aarch64-linux-android --release --features tauri/custom-protocol
```

Without `tauri/custom-protocol`, an installed release APK can try to load the development server at `http://localhost:1420/` and show a blank or error page.

## Signing

Keep signing material out of git. Do not commit keystores, local signing properties, or signing passwords.

Recommended local signing location:

```text
C:\Users\31445\Documents\pomodoro-study-local-signing
```

The repository ignores common signing files:

```gitignore
*.jks
*.keystore
key.properties
```

When signing is configured, document the local file names and required environment variables here without writing secrets into the repository.

## Manual APK Install

1. Build the APK.
2. Copy the generated APK to the Android phone or tablet.
3. Allow installing from the chosen file source on Android.
4. Install the APK.
5. Launch Pomodoro Study.
6. Run the manual acceptance checklist below.

## Manual Acceptance Checklist

Test on both phone and tablet:

1. Launches in Simplified Chinese by default.
2. Language switches to English and back.
3. Focus timer starts, pauses, resumes, and completes.
4. Interrupt saves an optional reason.
5. Short break and long break behavior works.
6. Current task selection affects completed pomodoro history.
7. Goals and tasks can be created, completed, and deleted.
8. Stats render today, last 7 days, goals, and interruptions.
9. Test sound plays the bundled `ding.wav`.
10. System notification appears at timer end when enabled.
11. Vibration runs at timer end when enabled.
12. Foreground notification setting can be toggled.
13. Sending the app to background during a timer and returning later corrects the remaining time.
14. JSON export creates a backup.
15. JSON import restores a backup.
16. Phone portrait, phone landscape, tablet portrait, and tablet landscape have no text or control overlap.
17. Button presses, selected tasks, route entrances, and progress feedback feel consistent with the desktop Acid motion system.
18. Android system "Remove animations" or reduced-motion settings do not leave essential information hidden.

## Environment Status

Last checked: 2026-06-18.

- Android project files have been generated under `src-tauri/gen/android`.
- Release APK verification used Android SDK `C:\Users\31445\AppData\Local\Android\Sdk`, NDK `27.0.12077973`, and JDK 17 from `C:\Users\31445\Documents\android-build-tools\jdk-17\jdk-17.0.19+10`.
- The verified test device was Lenovo `TB710FU`, Android 16 / SDK 36, `arm64-v8a`.
- The fixed Android APK links `libc++_shared.so` and uses packaged Tauri custom protocol assets in release mode.
- Local test signing uses a debug-style test key only. Use a production keystore before a public store release.

## Troubleshooting

- If `tauri android init` cannot find Android SDK or NDK, install the missing packages through Android Studio SDK Manager and reopen the terminal.
- If `adb` cannot see the device, enable Android developer options and USB debugging.
- If notifications do not appear, grant notification permission in Android system settings.
- If background reminders are unreliable, keep foreground notification enabled and check the device's battery optimization settings.
- If the APK will not install, uninstall an older build with a conflicting signature or rebuild with the same signing key.
