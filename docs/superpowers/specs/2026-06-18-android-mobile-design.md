# Pomodoro Study Android Mobile Design

**Date:** 2026-06-18
**Status:** Implemented baseline for Android v0.2.0
**Project root:** `C:\Users\31445\Documents\pomodoro-study`

## 1. Goal

Build an Android version of Pomodoro Study that keeps the desktop app's core experience while adapting the layout, touch targets, notifications, and packaging for phones and tablets.

The Android version should feel like the same product: black surfaces, acid-lime accents, hard industrial edges, goals, tasks, timer, statistics, settings, Simplified Chinese by default, English as an optional language, and JSON import/export for local backup.

## 2. Confirmed Scope

### In Scope

- Android only.
- Android 10+ as the default minimum target.
- Local APK install first.
- Signed release APK from the first Android build.
- Later upload of the APK to GitHub Releases.
- Phone and tablet support in the first version.
- Portrait and landscape layouts, with tablet landscape treated as a first-class layout.
- Timer, goals, tasks, stats, settings, language switching, sound, system notifications, vibration, and JSON import/export.
- Local SQLite storage per device.
- Design and maintenance documentation committed and pushed to GitHub.

### Out of Scope

- iOS.
- Google Play release.
- Desktop/mobile data sync.
- Cloud accounts or login.
- Rewriting the app as a separate native Android product.

## 3. Recommended Technical Route

Use **Tauri Android + the existing Svelte/SvelteKit frontend + Rust IPC + SQLite**.

This keeps one codebase for desktop and Android. The existing boundaries remain valid:

- Svelte owns UI, routing, timer state, i18n, themes, and responsive layout.
- Rust/Tauri owns SQLite, filesystem paths, notifications, sound, vibration, Android platform integration, and packaged resources.
- `src/lib/ipc.ts` remains the frontend boundary for persistent data and platform features.
- SQLite remains the durable local source of truth.

Alternative routes were considered:

- **Capacitor wrapper:** mature mobile plugin ecosystem, but it would replace the current Rust/Tauri IPC and persistence layer.
- **Native Kotlin Android:** strongest Android background control, but it would duplicate the UI and product logic.

The Tauri route is preferred because this app already has a working Tauri 2 architecture and the Android version should stay maintainable by future agents without splitting the product.

References for implementation planning:

- Tauri Android prerequisites: https://v2.tauri.app/start/prerequisites/
- Tauri Android signing: https://v2.tauri.app/distribute/sign/android/
- Tauri mobile plugin guidance: https://v2.tauri.app/develop/plugins/mobile/

## 4. Architecture

### 4.1 Shared Core

The Android build should reuse the current project structure:

```text
src/
  lib/
    components/
    stores/
    ipc.ts
    i18n.ts
    types.ts
  routes/
  themes/
src-tauri/
  src/
    commands/
    db/
    seed.rs
    state.rs
```

The main architectural rule is unchanged: frontend code must not write persistent app data directly. Any durable change goes through `src/lib/ipc.ts` and Rust commands.

### 4.2 Mobile Platform Layer

Add Android-specific capability behind platform-aware wrappers instead of spreading Android checks through route components.

Expected platform capabilities:

- System notification.
- Persistent foreground notification while a timer is active.
- Notification permission request and permission status.
- Sound playback using bundled resources.
- Vibration.
- File import/export for JSON backup files.
- Android build and signing configuration.

Desktop behavior must continue to use the existing desktop implementation. Shared frontend code should call one stable wrapper API and let the platform layer choose the desktop or Android implementation.

### 4.3 Timer Reliability Model

The frontend timer should continue to render live countdown state, but active sessions must be represented by absolute timestamps:

- Start time.
- Planned end time.
- Current phase: focus, short break, long break.
- Related task and goal identifiers when available.

When the app resumes from background, it should recompute remaining time from system time instead of trusting a suspended JavaScript timer. This is the main correctness guarantee.

During an active timer, Android should show a persistent foreground notification by default. This improves background reliability and makes the timer visible outside the app. The user may disable this in settings, with a warning that background reminders may be less reliable.

## 5. Phone And Tablet Layout

### 5.1 Visual Direction

Keep the current Acid Art / Industrial style:

- Black and near-black surfaces.
- Acid-lime accent.
- Hard borders.
- Square controls.
- No soft card-heavy mobile redesign.
- Touch-friendly spacing and control sizes.

The mobile version should look adapted, not simplified into a generic mobile template.

### 5.2 Phone Portrait

The first screen is **Timer Home**:

- Large countdown centered in the primary region.
- Current task summary below the countdown.
- Main controls near the thumb zone: start, pause, resume, interrupt, complete when relevant.
- Top-right small settings button.
- Bottom navigation with `Timer`, `Tasks`, and `Stats`.

Settings opens as its own screen. It contains language, timer lengths, notification, sound, vibration, import/export, foreground notification, permission guidance, and the GitHub link in small text near the bottom or another quiet empty area.

### 5.3 Phone Landscape

Phone landscape keeps the timer as the priority:

- Left side: countdown and phase.
- Right side: current task and quick task list.
- Navigation is compact or collapsible.
- Controls must not be squeezed by the short vertical height.

This layout should be usable, but it does not need the same density as tablet landscape.

### 5.4 Tablet Portrait

Tablet portrait uses more vertical space:

- Main timer area remains prominent.
- Task and goal summary can appear below or beside the timer depending on available width.
- Stats summaries may show more rows without requiring extra taps.

### 5.5 Tablet Landscape

Tablet landscape is a first-class layout and may resemble the desktop app:

- Left navigation rail.
- Center timer workspace.
- Right panel for current task, task list, goals, and today's summary.

This layout should make tablets feel like a productivity surface rather than an enlarged phone.

## 6. Feature Design

### 6.1 Timer

The Android timer keeps desktop behavior:

- Focus, short break, and long break phases.
- Long break after the configured number of completed focus sessions.
- Pause and resume.
- Interrupt with optional reason.
- Completion writes a durable pomodoro record.
- Current task selection affects pomodoro history and task progress.

On Android, timer completion should be visible through notification, sound, and vibration based on settings.

### 6.2 Goals And Tasks

Keep the full desktop structure:

- Goals with title, description, color, status.
- Tasks optionally linked to goals.
- Estimated pomodoros.
- Current task selection.
- Complete, delete, reorder, and filter behavior where practical.

The mobile change is layout and input ergonomics, not a simpler data model.

### 6.3 Stats

Keep desktop statistics, adapted for mobile readability:

- Today.
- Total focus.
- Streak or continuity summary.
- Last 7 days.
- By goal.
- Interruptions.

Phone screens can stack sections. Tablet screens can show multiple summaries at once.

### 6.4 Settings

Settings should include:

- Language: Simplified Chinese and English.
- Timer durations.
- Long break cycle.
- System notification toggle.
- Sound toggle.
- Test sound.
- Vibration toggle.
- Foreground notification toggle.
- JSON export.
- JSON import.
- GitHub link in small text.
- Permission and battery optimization guidance when Android restrictions affect reminders.

Default language is Simplified Chinese.

## 7. Android Platform Behavior

### 7.1 Notifications

Use separate settings for:

- System notification.
- Sound.
- Vibration.
- Foreground notification during active timer.

All are enabled by default.

Timer end should trigger the enabled reminder channels. The app should avoid silently failing when notification permission is missing; settings should show the permission state and offer a way to request it.

### 7.2 Sound

Reuse the bundled `ding.wav` resource as the default sound. The desktop "test sound" behavior must keep working, and Android must provide equivalent feedback when the user taps test sound.

### 7.3 Vibration

Vibration is Android-only in the first mobile version. The setting should be shown on Android and either hidden or disabled on desktop if the shared settings page is used.

### 7.4 File Import And Export

JSON import/export should use Android file picker or share/save behavior.

The data format should stay compatible with the existing `export_data` and `import_data` IPC commands. No new sync format is introduced.

### 7.5 Battery And Background Limits

Android may restrict background work depending on device settings and vendor battery policies. The design should handle this honestly:

- Active timer stores absolute timing information.
- App resume recalculates state from system time.
- Foreground notification is enabled by default.
- Settings explains permission or battery restrictions when detected or relevant.
- The first version does not attempt aggressive vendor-specific background bypasses.

## 8. Data Model

The existing SQLite schema remains the source of truth:

- `goals`
- `tasks`
- `pomodoros`
- `interrupts`
- `settings`

Settings values remain JSON strings. The persisted `language` setting remains `"zh-CN"` or `"en"` and defaults to `"zh-CN"`.

If Android requires new settings, add them through the existing settings table instead of creating a mobile-only configuration system.

Likely new setting keys:

- `notify.vibration`
- `notify.foreground`
- `android.notification_permission_seen`
- `android.battery_guidance_seen`

Only add these during implementation if they are actually needed.

## 9. Documentation Deliverables

The implementation phase should add or update:

- `docs/ANDROID_MOBILE.md`
  - Android environment setup.
  - Tauri Android initialization.
  - Local dev build.
  - Signed release APK build.
  - Installing APK on phone and tablet.
  - Common troubleshooting.
- `docs/AGENT_GUIDE.md`
  - Mobile maintenance rules.
  - Keep desktop behavior intact.
  - Keep Android features behind platform wrappers.
  - Do not bypass `src/lib/ipc.ts`.
- `docs/IPC_API.md`
  - Any new commands for vibration, foreground notification, file handling, or permission status.

This design document itself is the approved product and architecture direction for the Android work.

## 10. Build And Release

First Android release target:

- Signed release APK.
- Manual installation on the user's Android phone and tablet.
- GitHub Release upload after the APK is verified locally.

The first release should not require Google Play. Signing material should not be committed to the repository. Document where the local keystore lives and how to configure it through ignored local files or environment variables.

## 11. Testing And Acceptance

### 11.1 Automated Checks

Run the existing checks during implementation:

- `pnpm check`
- `pnpm test`
- `pnpm lint`
- Rust tests where applicable.

Desktop behavior must be checked after mobile changes.

### 11.2 Manual Android Acceptance

Test on phone and tablet:

- Install signed APK.
- Launch app in Simplified Chinese by default.
- Switch language to English and back.
- Start, pause, resume, complete a focus timer.
- Interrupt a focus timer with a reason.
- Confirm short break and long break behavior.
- Select a current task and complete a pomodoro.
- Create, complete, and delete tasks.
- View today and last 7 days stats.
- Tap test sound and hear the bundled sound.
- Receive system notification at timer end.
- Feel vibration at timer end when enabled.
- Send app to background during a timer, return later, and confirm time is corrected.
- Confirm foreground notification appears during active timer when enabled.
- Export JSON.
- Reset/import data and confirm records return.
- Rotate phone portrait/landscape.
- Rotate tablet portrait/landscape.
- Confirm text and controls do not overlap.

### 11.3 Release Acceptance

The Android version is ready for a first release when:

- Signed APK builds successfully.
- APK installs on the user's phone and tablet.
- Core desktop app still passes checks.
- Android manual acceptance passes.
- GitHub Release contains the APK and basic install notes.
- Android maintenance documentation is committed.

## 12. Future Notes

Possible future work after the first APK:

- More notification sounds.
- Better Android permission onboarding.
- Home screen shortcut or quick action.
- Optional data sync.
- Play Store packaging.
- Tablet-specific keyboard shortcuts.

These are intentionally outside the first Android build.
