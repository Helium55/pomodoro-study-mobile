# Architecture

## Process Model

- Rust Tauri shell owns SQLite, filesystem paths, notifications, sound, and window attention.
- Svelte 5/SvelteKit owns all UI, timer state, routing, and theme switching.
- `src/lib/ipc.ts` is the typed frontend boundary for every Tauri command.
- SQLite is the durable source of truth. The browser fallback in `ipc.ts` exists only so the UI can run in a non-Tauri preview.

## Folders

```text
src/
  lib/
    components/  reusable UI
    stores/      Svelte 5 rune stores
    ipc.ts       frontend IPC wrapper
    time.ts      timer formatting helpers
  routes/        focus, tasks, stats, theme, settings
  themes/        token contract and theme packs
src-tauri/
  src/
    commands/    IPC command modules
    db/          SQLite connection and migrations
    error.rs
    seed.rs
    state.rs
```

## Boundaries

- Frontend code never writes persistent app data directly; it calls IPC.
- Rust commands never call Svelte; they return data and the frontend reacts.
- Components consume CSS tokens only. Theme files define concrete colors.
- Database migrations are append-only.
