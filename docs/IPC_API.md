# IPC API

Frontend wrappers live in `src/lib/ipc.ts`.

## Goals

| Command | Params | Returns |
|---|---|---|
| `create_goal` | `title`, `description?`, `color?` | `Goal` |
| `list_goals` | `includeArchived` | `Goal[]` |
| `update_goal` | `id`, patch fields | `void` |
| `archive_goal` | `id` | `void` |
| `delete_goal` | `id` | `void` |

## Tasks

| Command | Params | Returns |
|---|---|---|
| `create_task` | `goalId?`, `title`, `estimatedPomos?` | `Task` |
| `list_tasks` | `goalId?`, `includeDone` | `Task[]` |
| `update_task` | `id`, patch fields | `void` |
| `complete_task` | `id` | `void` |
| `delete_task` | `id` | `void` |
| `reorder_tasks` | `orderedIds` | `void` |

## Pomodoros

| Command | Params | Returns |
|---|---|---|
| `start_pomodoro` | `taskId?`, `plannedSecs` | `Pomodoro` |
| `complete_pomodoro` | `id`, `actualSecs` | `void` |
| `interrupt_pomodoro` | `id`, `reason?`, `actualSecs`, `abandoned` | `void` |
| `list_pomodoros_today` | none | `Pomodoro[]` |

## Settings, Stats, Notifications, Data

| Command | Params | Returns |
|---|---|---|
| `get_setting` | `key` | `string?` |
| `set_setting` | `key`, `value` | `void` |
| `get_all_settings` | none | `Record<string, string>` |
| `get_stats` | none | `StatsSummary` |
| `notify_system` | `title`, `body` | `void` |
| `notify_sound` | `soundFile` | `void` (bundled resource under `assets/`; falls back to `ding.wav`) |
| `notify_vibration` | none | `void` |
| `set_foreground_timer` | `phase`, `title`, `body`, `endsAtMs` | `void` |
| `clear_foreground_timer` | none | `void` |
| `notify_focus_window` | none | `void` |
| `notify_taskbar_flash` | none | `void` |
| `export_data` | none | `string` |
| `import_data` | `jsonText` | `void` |
| `reset_data` | none | `void` |
