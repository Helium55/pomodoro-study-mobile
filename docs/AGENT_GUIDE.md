# Agent Maintenance Guide

## Hard Rules

1. Do not write hex colors, rounded corners, or blurred shadows into component CSS.
2. Do not bypass `src/lib/ipc.ts` for persistent app data.
3. Treat completed `pomodoros` rows as historical facts.
4. Migrations are append-only.
5. Settings values are JSON-encoded strings.
6. Keep Android platform behavior behind `src/lib/ipc.ts` or a focused platform helper.
7. Do not commit Android signing keys, keystores, local signing properties, or signing passwords.
8. Test phone portrait, phone landscape, tablet portrait, and tablet landscape before Android release.

## Common Tasks

### Add A Setting

1. Add the default in `src-tauri/src/seed.rs`.
2. Add the field to `SettingsState`.
3. Add a key in `KEY_MAP` in `settings.svelte.ts`.
4. Add a control in `src/routes/settings/+page.svelte`.
5. If the setting affects visible text, add copy in `src/lib/i18n.ts`.

### Add A Statistic

1. Extend `StatsSummary` in `src-tauri/src/commands/stats.rs`.
2. Mirror the type in `src/lib/types.ts`.
3. Render it in `src/routes/stats/+page.svelte`.
4. Update `docs/IPC_API.md`.

### Add An IPC Command

1. Implement it under `src-tauri/src/commands/`.
2. Register it in `tauri::generate_handler!` in `src-tauri/src/lib.rs`.
3. Add a wrapper in `src/lib/ipc.ts`.
4. Document it in `docs/IPC_API.md`.

## Manual Acceptance Checklist

1. Start a focus session with no task; let it finish; confirm a break starts.
2. Start, pause, resume; confirm the countdown resumes correctly.
3. Start, interrupt, enter a reason; confirm the session returns to idle and the interrupt appears in stats.
4. Add a goal, add a task, select it, complete a focus; confirm task progress increments.
5. Complete four focus sessions; confirm the next break is long.
6. Switch theme; confirm Acid remains active and Synthwave is disabled as coming soon.
7. Switch language between Simplified Chinese and English; confirm visible navigation and Settings copy changes.
8. Click test sound in Settings; confirm a status message appears and packaged builds include `src-tauri/assets/ding.wav`.
9. Change settings; restart; confirm settings persist.
10. Export JSON, reset data, import JSON; confirm data returns.
11. On phone portrait, confirm bottom navigation does not cover Settings data actions.
12. On phone landscape, confirm the timer and controls fit above bottom navigation.
13. On tablet landscape, confirm the side rail remains visible and the timer workspace has no horizontal overflow.
