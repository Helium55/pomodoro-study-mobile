# Data Model

Schema source of truth: `src-tauri/src/db/migrations/0001_init.sql`.

## Tables

### `goals`

Long-term study goals. `status` is `active`, `archived`, or `done`.

### `tasks`

Concrete work items. A task may belong to a goal or have `goal_id IS NULL`. `estimated_pomos` powers the progress display.

### `pomodoros`

Fact table for focus sessions. Completed rows should not be rewritten for ordinary edits. `goal_id` is intentionally duplicated so historical stats survive task reassignment.

`date_local` is a `YYYY-MM-DD` string assigned when a session starts. Statistics group by this value.

### `interrupts`

Free-text interruptions attached to a pomodoro. Rows cascade when the pomodoro is removed during data reset/import.

### `settings`

Key-value table. Values are JSON strings so booleans, numbers, and strings round-trip cleanly.

The persisted `language` setting is `"zh-CN"` or `"en"` and defaults to Simplified Chinese. The bundled notification sound defaults to `"ding.wav"`.

## Migrations

Add new migrations as `src-tauri/src/db/migrations/000N_description.sql` and append the version to `MIGRATIONS` in `src-tauri/src/db/migrations.rs`. Do not edit an already released migration.
