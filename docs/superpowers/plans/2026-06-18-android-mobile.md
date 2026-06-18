# Android Mobile Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first Android-ready version of Pomodoro Study with phone/tablet layouts, Android-facing reminder settings, timestamp-correct timer behavior, and maintainable Android build documentation.

**Architecture:** Keep the existing Tauri 2 + Svelte 5 + SQLite architecture. Add a small platform layer around runtime detection, notifications, vibration, foreground timer state, and Android file guidance while preserving `src/lib/ipc.ts` as the frontend boundary.

**Tech Stack:** Svelte 5, SvelteKit static adapter, TypeScript, Vitest, Tauri 2, Rust, SQLite, pnpm.

---

## File Map

- Modify `src/lib/types.ts`: add Android reminder settings to `SettingsState`.
- Modify `src/lib/stores/settings.svelte.ts`: load, persist, and default `notifyVibration` and `notifyForeground`.
- Modify `src/lib/ipc.ts`: add memory defaults and wrapper methods for vibration and foreground timer hooks.
- Create `src/lib/platform.ts`: centralize runtime/platform detection and CSS class targets.
- Create `src/lib/platform.test.ts`: cover runtime classification for browser fallback, Android UA, tablet width, and desktop.
- Modify `src/lib/stores/timer.svelte.ts`: use absolute timestamps, resume correction, vibration hook, and foreground hook.
- Modify `src/routes/+layout.svelte`: add mobile/tablet classes and safe-area-aware app shell.
- Modify `src/lib/components/Sidebar.svelte`: make phone bottom nav, tablet rail, and hide Theme from phone bottom nav.
- Modify `src/routes/focus/+page.svelte`: phone Timer Home and tablet landscape workspace.
- Modify `src/routes/tasks/+page.svelte`: mobile-friendly forms and tablet two-pane layout.
- Modify `src/routes/stats/+page.svelte`: mobile stacked stats and tablet landscape summaries.
- Modify `src/routes/settings/+page.svelte`: add vibration and foreground notification controls plus Android guidance.
- Modify `src/lib/i18n.ts`: add copy for new Android settings and guidance.
- Modify `src-tauri/src/seed.rs`: add default settings.
- Modify `src-tauri/src/commands/notify.rs`: add desktop-safe no-op commands for vibration and foreground timer state.
- Modify `src-tauri/src/lib.rs`: register new commands.
- Modify `src-tauri/capabilities/default.json`: grant permissions for new commands.
- Modify `docs/AGENT_GUIDE.md`: add Android maintenance rules and manual mobile checklist.
- Create `docs/ANDROID_MOBILE.md`: Android setup, build, signing, install, and troubleshooting guide.
- Modify `docs/IPC_API.md`: document new notification/timer platform commands.

---

## Task 1: Baseline Verification And Toolchain Discovery

**Files:**
- Read: `package.json`
- Read: `src-tauri/tauri.conf.json`
- Read: `src-tauri/Cargo.toml`

- [ ] **Step 1: Verify current repository state**

Run:

```powershell
git status --short --branch
```

Expected: the branch is clean or only contains this plan if it has not been committed yet.

- [ ] **Step 2: Run current frontend checks**

Run:

```powershell
corepack pnpm check
corepack pnpm test
corepack pnpm lint
```

Expected: all commands exit 0 before Android work begins. If a command fails because of pre-existing code, record the failure in the implementation notes and keep the Android edits separate.

- [ ] **Step 3: Run current Rust tests**

Run:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml
```

Expected: all Rust tests exit 0 before Android work begins.

- [ ] **Step 4: Discover local Tauri Android commands**

Run:

```powershell
corepack pnpm tauri android --help
```

Expected: the CLI prints Android subcommands. If Android SDK or NDK is missing, do not install globally from the plan; document the missing requirement in `docs/ANDROID_MOBILE.md`.

- [ ] **Step 5: Commit baseline-only documentation if needed**

Run only if this plan file is uncommitted:

```powershell
git add docs/superpowers/plans/2026-06-18-android-mobile.md
git commit -m "docs: add android mobile implementation plan"
```

Expected: one docs-only commit.

---

## Task 2: Add Platform Detection And Android Reminder Settings

**Files:**
- Create: `src/lib/platform.ts`
- Create: `src/lib/platform.test.ts`
- Modify: `src/lib/types.ts`
- Modify: `src/lib/stores/settings.svelte.ts`
- Modify: `src/lib/ipc.ts`
- Modify: `src-tauri/src/seed.rs`

- [ ] **Step 1: Add failing tests for platform classification**

Create `src/lib/platform.test.ts` with tests that expect:

```ts
import { describe, expect, it } from 'vitest'
import { getPlatformProfile } from './platform'

describe('getPlatformProfile', () => {
  it('detects Android phones from the user agent', () => {
    const profile = getPlatformProfile({
      userAgent: 'Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36',
      width: 390,
      hasTauriInternals: true,
    })

    expect(profile.os).toBe('android')
    expect(profile.formFactor).toBe('phone')
    expect(profile.isMobileLayout).toBe(true)
  })

  it('detects Android tablets from width', () => {
    const profile = getPlatformProfile({
      userAgent: 'Mozilla/5.0 (Linux; Android 14; Tablet) AppleWebKit/537.36',
      width: 1100,
      hasTauriInternals: true,
    })

    expect(profile.os).toBe('android')
    expect(profile.formFactor).toBe('tablet')
    expect(profile.isMobileLayout).toBe(false)
  })

  it('keeps desktop as desktop', () => {
    const profile = getPlatformProfile({
      userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)',
      width: 1280,
      hasTauriInternals: true,
    })

    expect(profile.os).toBe('desktop')
    expect(profile.formFactor).toBe('desktop')
    expect(profile.isMobileLayout).toBe(false)
  })
})
```

- [ ] **Step 2: Run the new test and verify it fails**

Run:

```powershell
corepack pnpm vitest run src/lib/platform.test.ts
```

Expected: failure because `src/lib/platform.ts` does not exist.

- [ ] **Step 3: Implement `src/lib/platform.ts`**

Create a small pure helper plus browser wrapper:

```ts
export type PlatformOs = 'android' | 'desktop' | 'browser'
export type FormFactor = 'phone' | 'tablet' | 'desktop'

export interface PlatformInput {
  userAgent: string
  width: number
  hasTauriInternals: boolean
}

export interface PlatformProfile {
  os: PlatformOs
  formFactor: FormFactor
  isTauri: boolean
  isAndroid: boolean
  isMobileLayout: boolean
}

export function getPlatformProfile(input: PlatformInput): PlatformProfile {
  const isAndroid = /Android/i.test(input.userAgent)
  const isTauri = input.hasTauriInternals
  const os: PlatformOs = isAndroid ? 'android' : isTauri ? 'desktop' : 'browser'
  const formFactor: FormFactor = isAndroid
    ? input.width >= 840
      ? 'tablet'
      : 'phone'
    : 'desktop'

  return {
    os,
    formFactor,
    isTauri,
    isAndroid,
    isMobileLayout: formFactor === 'phone',
  }
}

export function readPlatformProfile(): PlatformProfile {
  if (typeof window === 'undefined') {
    return getPlatformProfile({ userAgent: '', width: 1280, hasTauriInternals: false })
  }

  return getPlatformProfile({
    userAgent: window.navigator.userAgent,
    width: window.innerWidth,
    hasTauriInternals: '__TAURI_INTERNALS__' in window,
  })
}
```

- [ ] **Step 4: Add reminder settings types**

In `src/lib/types.ts`, add to `SettingsState`:

```ts
notifyVibration: boolean
notifyForeground: boolean
```

- [ ] **Step 5: Add defaults and key mapping**

In `src/lib/stores/settings.svelte.ts`, add defaults:

```ts
notifyVibration: true,
notifyForeground: true,
```

Add `KEY_MAP` entries:

```ts
notifyVibration: 'notify.vibration',
notifyForeground: 'notify.foreground',
```

In `load()`, assign:

```ts
this.state.notifyVibration = Boolean(readSetting(values[KEY_MAP.notifyVibration], true))
this.state.notifyForeground = Boolean(readSetting(values[KEY_MAP.notifyForeground], true))
```

- [ ] **Step 6: Add browser fallback defaults**

In `src/lib/ipc.ts`, add:

```ts
'notify.vibration': 'true',
'notify.foreground': 'true',
```

- [ ] **Step 7: Add SQLite seed defaults and adjust tests**

In `src-tauri/src/seed.rs`, add:

```rust
("notify.vibration", "true"),
("notify.foreground", "true"),
```

Update existing count assertions from `12` to `14`.

- [ ] **Step 8: Run focused tests**

Run:

```powershell
corepack pnpm vitest run src/lib/platform.test.ts
cargo test --manifest-path src-tauri/Cargo.toml seed
```

Expected: platform tests pass and seed tests pass.

- [ ] **Step 9: Commit**

Run:

```powershell
git add src/lib/platform.ts src/lib/platform.test.ts src/lib/types.ts src/lib/stores/settings.svelte.ts src/lib/ipc.ts src-tauri/src/seed.rs
git commit -m "feat(android): add platform profile and mobile reminder settings"
```

Expected: one focused commit.

---

## Task 3: Add IPC Wrappers For Vibration And Foreground Timer State

**Files:**
- Modify: `src/lib/ipc.ts`
- Modify: `src-tauri/src/commands/notify.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/capabilities/default.json`
- Modify: `docs/IPC_API.md`

- [ ] **Step 1: Add frontend wrappers and memory no-ops**

In `memoryInvoke()` in `src/lib/ipc.ts`, add no-op cases:

```ts
case 'notify_vibration':
case 'set_foreground_timer':
case 'clear_foreground_timer':
  return undefined as T
```

In `export const ipc`, add:

```ts
notifyVibration: () => call<void>('notify_vibration'),
setForegroundTimer: (args: {
  phase: string
  title: string
  body: string
  endsAtMs: number
}) => call<void>('set_foreground_timer', args),
clearForegroundTimer: () => call<void>('clear_foreground_timer'),
```

- [ ] **Step 2: Add Rust command stubs**

In `src-tauri/src/commands/notify.rs`, add:

```rust
#[tauri::command]
pub fn notify_vibration() -> AppResult<()> {
    Ok(())
}

#[tauri::command]
pub fn set_foreground_timer(
    _phase: String,
    _title: String,
    _body: String,
    _ends_at_ms: i64,
) -> AppResult<()> {
    Ok(())
}

#[tauri::command]
pub fn clear_foreground_timer() -> AppResult<()> {
    Ok(())
}
```

The first implementation is desktop-safe and Android-ready at the IPC boundary. A later Android-native plugin can replace the internals without changing frontend callers.

- [ ] **Step 3: Register commands**

In `src-tauri/src/lib.rs`, add these commands to `tauri::generate_handler!`:

```rust
commands::notify::notify_vibration,
commands::notify::set_foreground_timer,
commands::notify::clear_foreground_timer,
```

- [ ] **Step 4: Update capability allowlist**

In `src-tauri/capabilities/default.json`, include:

```json
"core:default",
"opener:default"
```

No extra permission entry is required for local app commands registered through `generate_handler!`; keep the file valid JSON and unchanged beyond formatting if Tauri accepts the current command model.

- [ ] **Step 5: Document IPC**

In `docs/IPC_API.md`, add rows:

```markdown
| `notify_vibration` | none | `void` |
| `set_foreground_timer` | `phase`, `title`, `body`, `endsAtMs` | `void` |
| `clear_foreground_timer` | none | `void` |
```

- [ ] **Step 6: Verify**

Run:

```powershell
corepack pnpm check
cargo test --manifest-path src-tauri/Cargo.toml
```

Expected: TypeScript and Rust both pass.

- [ ] **Step 7: Commit**

Run:

```powershell
git add src/lib/ipc.ts src-tauri/src/commands/notify.rs src-tauri/src/lib.rs src-tauri/capabilities/default.json docs/IPC_API.md
git commit -m "feat(android): add reminder platform commands"
```

Expected: one focused commit.

---

## Task 4: Make Timer State Timestamp-Correct

**Files:**
- Modify: `src/lib/stores/timer.svelte.ts`

- [ ] **Step 1: Add timestamp fields**

Add fields to `TimerStore`:

```ts
endsAtMs = 0
phaseStartedAtMs = 0
```

- [ ] **Step 2: Set absolute end timestamps when starting focus**

In `startFocus()`, after `this.startMs = Date.now()`, set:

```ts
this.phaseStartedAtMs = this.startMs
this.endsAtMs = this.startMs + duration * 1000
await this.syncForegroundTimer()
```

- [ ] **Step 3: Adjust pause and resume**

In `pause()`, after setting `pausedAt`, call:

```ts
void ipc.clearForegroundTimer().catch(() => undefined)
```

In `resume()`, after updating `pausedMs`, adjust:

```ts
const pausedFor = Date.now() - this.pausedAt
this.endsAtMs += pausedFor
```

Then call:

```ts
void this.syncForegroundTimer()
```

- [ ] **Step 4: Recompute remaining time from absolute timestamps**

Change `tick()` so running timers use:

```ts
this.remainingSecs = Math.max(0, Math.ceil((this.endsAtMs - Date.now()) / 1000))
```

Keep `finishPhase()` when remaining reaches zero.

- [ ] **Step 5: Add foreground sync helper**

Add method:

```ts
async syncForegroundTimer() {
  if (!settings.state.notifyForeground || this.status !== 'running' || this.phase === 'idle') {
    await ipc.clearForegroundTimer().catch(() => undefined)
    return
  }

  const copy = getCopy(settings.state.language)
  const phaseLabel = copy.timer.phase[this.phase]
  await ipc
    .setForegroundTimer({
      phase: this.phase,
      title: phaseLabel,
      body: tasks.selected?.title ?? phaseLabel,
      endsAtMs: this.endsAtMs,
    })
    .catch(() => undefined)
}
```

- [ ] **Step 6: Add vibration at phase completion**

In `finishPhase()`, after sound notification:

```ts
if (settings.state.notifyVibration) {
  await ipc.notifyVibration().catch(() => undefined)
}
```

- [ ] **Step 7: Clear foreground state on reset and interrupt**

In `reset()`, before state reset ends:

```ts
void ipc.clearForegroundTimer().catch(() => undefined)
```

In `interrupt()`, after `await tasks.load()`:

```ts
await ipc.clearForegroundTimer().catch(() => undefined)
```

- [ ] **Step 8: Start break with absolute timestamps**

In `startBreak()`, after setting `this.startMs = Date.now()`, set:

```ts
this.phaseStartedAtMs = this.startMs
this.endsAtMs = this.startMs + this.durationSecs * 1000
void this.syncForegroundTimer()
```

- [ ] **Step 9: Verify**

Run:

```powershell
corepack pnpm check
corepack pnpm test
```

Expected: TypeScript and unit tests pass.

- [ ] **Step 10: Commit**

Run:

```powershell
git add src/lib/stores/timer.svelte.ts
git commit -m "feat(android): make timer resume from absolute time"
```

Expected: one focused commit.

---

## Task 5: Add Mobile And Tablet App Shell

**Files:**
- Modify: `src/routes/+layout.svelte`
- Modify: `src/lib/components/Sidebar.svelte`

- [ ] **Step 1: Add platform profile to layout**

In `src/routes/+layout.svelte`, import and store `readPlatformProfile()` on mount. Apply classes:

```svelte
<div
  class="app-shell"
  class:android={profile.isAndroid}
  class:phone={profile.formFactor === 'phone'}
  class:tablet={profile.formFactor === 'tablet'}
>
```

- [ ] **Step 2: Make shell safe-area-aware**

Update layout CSS so phone content leaves space for bottom navigation:

```css
.app-shell.phone {
  min-height: 100dvh;
}

.app-shell.phone .content {
  height: calc(100dvh - 64px);
  padding-bottom: env(safe-area-inset-bottom);
}

@media (width <= 760px) {
  .app-shell {
    flex-direction: column;
  }

  .content {
    height: calc(100dvh - 64px);
  }
}
```

- [ ] **Step 3: Convert phone navigation to bottom bar**

In `Sidebar.svelte`, keep all items for desktop/tablet, but hide `theme` from the phone bottom nav with CSS:

```css
@media (width <= 760px) {
  .sidebar {
    position: fixed;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 20;
    border-top: 1px solid var(--color-border);
    border-bottom: 0;
    padding-bottom: env(safe-area-inset-bottom);
  }

  .brand {
    display: none;
  }

  nav {
    grid-template-columns: repeat(4, 1fr);
  }

  a[href$='/theme'] {
    display: none;
  }
}
```

- [ ] **Step 4: Preserve tablet rail**

Add a tablet/large media rule that keeps the current side rail at widths above 760px.

- [ ] **Step 5: Verify responsive shell**

Run:

```powershell
corepack pnpm check
```

Expected: no Svelte or TypeScript errors.

- [ ] **Step 6: Commit**

Run:

```powershell
git add src/routes/+layout.svelte src/lib/components/Sidebar.svelte
git commit -m "feat(android): adapt app shell for phone and tablet"
```

Expected: one focused commit.

---

## Task 6: Adapt Focus, Tasks, Stats, And Settings Layouts

**Files:**
- Modify: `src/routes/focus/+page.svelte`
- Modify: `src/routes/tasks/+page.svelte`
- Modify: `src/routes/stats/+page.svelte`
- Modify: `src/routes/settings/+page.svelte`
- Modify: `src/lib/i18n.ts`

- [ ] **Step 1: Add Android settings copy**

In `src/lib/i18n.ts`, add settings copy keys for both languages:

```ts
vibration: 'Vibration',
foregroundNotification: 'Foreground notification',
androidGuidance: 'Keep this on for more reliable Android reminders.',
```

Use equivalent Simplified Chinese strings in the `zh-CN` branch.

- [ ] **Step 2: Add settings controls**

In `settings/+page.svelte`, add two checkbox labels in the notification section:

```svelte
<label class="check">
  <input
    type="checkbox"
    checked={settings.state.notifyVibration}
    onchange={(event) =>
      settings.update('notifyVibration', (event.target as HTMLInputElement).checked)}
  />
  {copy.settings.vibration}
</label>

<label class="check">
  <input
    type="checkbox"
    checked={settings.state.notifyForeground}
    onchange={(event) =>
      settings.update('notifyForeground', (event.target as HTMLInputElement).checked)}
  />
  {copy.settings.foregroundNotification}
</label>
<p class="android-guidance">{copy.settings.androidGuidance}</p>
```

- [ ] **Step 3: Improve focus phone layout**

In `focus/+page.svelte`, add phone-specific CSS:

```css
@media (width <= 760px) {
  .focus-page {
    grid-template-rows: auto auto minmax(260px, 1fr) auto auto;
  }

  .top {
    padding: 16px;
  }

  .timer-wrap {
    padding: 28px 14px;
  }

  .controls {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .controls button {
    min-height: 56px;
  }
}

@media (orientation: landscape) and (width <= 900px) {
  .focus-page {
    grid-template-columns: minmax(0, 1.2fr) minmax(260px, 0.8fr);
    grid-template-rows: auto 1fr auto;
  }

  .timer-wrap {
    grid-row: 1 / 4;
  }
}
```

- [ ] **Step 4: Improve tasks mobile forms**

In `tasks/+page.svelte`, keep two columns on tablet and single column on phone. Ensure form controls have `min-height: 44px` and no text overlap.

- [ ] **Step 5: Improve stats mobile density**

In `stats/+page.svelte`, stack stat cards on phone and keep two-column panels on tablet landscape. Ensure long goal names truncate with ellipsis.

- [ ] **Step 6: Verify frontend**

Run:

```powershell
corepack pnpm check
corepack pnpm test
corepack pnpm lint
```

Expected: all frontend checks pass.

- [ ] **Step 7: Commit**

Run:

```powershell
git add src/routes/focus/+page.svelte src/routes/tasks/+page.svelte src/routes/stats/+page.svelte src/routes/settings/+page.svelte src/lib/i18n.ts
git commit -m "feat(android): refine mobile and tablet layouts"
```

Expected: one focused commit.

---

## Task 7: Add Android Build Documentation

**Files:**
- Create: `docs/ANDROID_MOBILE.md`
- Modify: `docs/AGENT_GUIDE.md`

- [ ] **Step 1: Create Android maintenance guide**

Create `docs/ANDROID_MOBILE.md` with these sections:

```markdown
# Android Mobile Guide

## Scope

Pomodoro Study supports Android 10+ through Tauri Android. The first distribution target is a signed APK for local install and GitHub Releases.

## Required Tools

- Node.js and pnpm through Corepack.
- Rust toolchain.
- Android Studio.
- Android SDK Platform for Android 10+.
- Android SDK Build Tools.
- Android NDK installed through Android Studio.
- Java runtime compatible with the installed Android Gradle plugin.

## Commands

```powershell
corepack pnpm install
corepack pnpm tauri android init
corepack pnpm tauri android dev
corepack pnpm tauri android build
```

## Signing

Keep keystores out of git. Store local signing files under an ignored local path such as `C:\Users\31445\Documents\pomodoro-study-local-signing`.

## Manual Install

Copy the generated APK to the Android phone or tablet, allow installing from the chosen file source, and install the APK.

## Acceptance Checklist

Run the checklist from `docs/superpowers/specs/2026-06-18-android-mobile-design.md`.
```
```

- [ ] **Step 2: Update agent guide**

Add Android maintenance rules to `docs/AGENT_GUIDE.md`:

```markdown
## Android Maintenance Rules

1. Keep mobile platform behavior behind `src/lib/ipc.ts` or a focused platform helper.
2. Do not bypass SQLite or write durable app data from route components.
3. Keep desktop behavior working after mobile changes.
4. Do not commit Android signing keys, keystores, or local signing passwords.
5. Test phone portrait, phone landscape, tablet portrait, and tablet landscape before release.
```

- [ ] **Step 3: Commit**

Run:

```powershell
git add docs/ANDROID_MOBILE.md docs/AGENT_GUIDE.md
git commit -m "docs: add android mobile maintenance guide"
```

Expected: one docs commit.

---

## Task 8: Attempt Android Initialization And Record Environment Status

**Files:**
- Modify if generated successfully: `src-tauri/gen/android/**`
- Modify if needed: `src-tauri/tauri.conf.json`
- Modify if needed: `.gitignore`
- Modify: `docs/ANDROID_MOBILE.md`

- [ ] **Step 1: Run Android initialization**

Run:

```powershell
corepack pnpm tauri android init
```

Expected: if Android tooling is installed, Tauri generates `src-tauri/gen/android`. If tooling is missing, capture the exact missing tool name in `docs/ANDROID_MOBILE.md`.

- [ ] **Step 2: Verify generated files are appropriate**

Run:

```powershell
git status --short
```

Expected: generated Android project files appear only if initialization succeeds. Do not commit local signing files.

- [ ] **Step 3: Add ignore entries for signing artifacts**

If not already covered, add to `.gitignore`:

```gitignore
# Android signing secrets
*.jks
*.keystore
key.properties
```

- [ ] **Step 4: Build frontend and desktop**

Run:

```powershell
corepack pnpm build
corepack pnpm tauri build
```

Expected: desktop build still succeeds after mobile changes.

- [ ] **Step 5: Attempt Android build if toolchain exists**

Run only if `android init` succeeded:

```powershell
corepack pnpm tauri android build
```

Expected: APK build succeeds or fails with a documented environment/toolchain message.

- [ ] **Step 6: Commit generated Android files or documentation status**

If Android project generated:

```powershell
git add src-tauri/gen/android .gitignore docs/ANDROID_MOBILE.md
git commit -m "build(android): initialize tauri android project"
```

If environment is missing and only docs changed:

```powershell
git add docs/ANDROID_MOBILE.md .gitignore
git commit -m "docs: record android toolchain setup status"
```

Expected: one commit that accurately reflects the local result.

---

## Task 9: Final Verification And Push

**Files:**
- Read: all changed files through `git diff --stat`

- [ ] **Step 1: Run full verification**

Run:

```powershell
corepack pnpm check
corepack pnpm test
corepack pnpm lint
cargo test --manifest-path src-tauri/Cargo.toml
```

Expected: all checks pass, or any environment-only Android build limitation is documented.

- [ ] **Step 2: Review final diff**

Run:

```powershell
git status --short --branch
git log --oneline -8
```

Expected: working tree is clean after commits, with focused Android commits on top of `main`.

- [ ] **Step 3: Push**

Run:

```powershell
git push origin HEAD:main
```

Expected: GitHub `main` advances to the final Android implementation commit.

---

## Self-Review

- Spec coverage: the plan covers Android-only scope, Android 10+ documentation, signed APK guidance, phone/tablet layouts, landscape and portrait behavior, SQLite/local data boundaries, vibration/foreground settings, timestamp-correct timer behavior, JSON import/export documentation, and release acceptance.
- Placeholders: no `TBD`, no empty implementation sections, and no unspecified test commands.
- Type consistency: `notifyVibration`, `notifyForeground`, `notify_vibration`, `set_foreground_timer`, and `clear_foreground_timer` are named consistently across TypeScript, Rust, IPC docs, and settings.
