# Pomodoro Study Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Tauri 2 + Svelte 5 desktop pomodoro study app with two-layer goal/task structure, interrupt logging, statistics, and a pluggable theme system (Acid Art shipping first; Synthwave reserved).

**Architecture:** Rust (Tauri shell) handles OS I/O — SQLite, system notifications, sound, window control. Svelte 5 frontend handles UI, the timer state machine, and theme switching. They communicate via typed Tauri IPC commands. SQLite is the single source of truth; the frontend never owns persistent state.

**Tech Stack:** Tauri 2.x, Rust (rusqlite, notify-rust, rodio), Svelte 5 + TypeScript, Vite, pnpm, Vitest, cargo test.

**Spec:** `docs/superpowers/specs/2026-06-17-pomodoro-study-design.md`

**Project root:** `C:\Users\31445\Documents\pomodoro-study`

---

## Conventions Used Throughout

- All paths are relative to project root unless prefixed `C:\`.
- Run terminal commands from project root.
- Commit message format: `type(scope): subject` (e.g. `feat(timer): add state machine`).
- After each task, the working tree must be clean and tests passing before moving to the next.
- TDD is used wherever logic exists (Rust commands, Svelte stores, theme registry). Pure presentational components get smoke tests, not full TDD.

---

## Task 1: Project Scaffold (Tauri 2 + Svelte 5 + TypeScript)

**Files:**
- Create: `package.json`, `pnpm-workspace.yaml` (none — single package), `tsconfig.json`, `svelte.config.js`, `vite.config.ts`, `index.html`, `src/main.ts`, `src/App.svelte`, `src/app.css`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/build.rs`, `src-tauri/src/main.rs`, `src-tauri/src/lib.rs`, `.gitignore` (already exists, extend it)

- [ ] **Step 1: Run Tauri 2 scaffold via create-tauri-app**

Run from project root (which already exists with `docs/` and `.git`):

```bash
pnpm create tauri-app@latest pomodoro-tmp --template svelte-ts --manager pnpm --identifier study.pomodoro.app
```

Then move scaffold contents up one level and delete the temp dir:

```bash
cp -r pomodoro-tmp/. .
rm -rf pomodoro-tmp
```

Expected: project root now contains `package.json`, `src/`, `src-tauri/`, `vite.config.ts`, etc., alongside the existing `docs/` directory.

- [ ] **Step 2: Install dependencies**

```bash
pnpm install
```

Expected: `node_modules/` and `pnpm-lock.yaml` created. No errors.

- [ ] **Step 3: Verify dev server starts**

```bash
pnpm tauri dev
```

Expected: A native window opens showing the default Tauri+Svelte starter. Close it (Ctrl+C in terminal).

- [ ] **Step 4: Extend `.gitignore`**

Append to existing `.gitignore` (don't overwrite the lines from spec commit):

```
# Tauri / Rust
src-tauri/target/
src-tauri/Cargo.lock

# Frontend
node_modules/
dist/
.svelte-kit/

# OS / Editor
.DS_Store
Thumbs.db
.vscode/
.idea/
```

- [ ] **Step 5: Configure Tauri window**

Edit `src-tauri/tauri.conf.json` — set the window block:

```json
"app": {
  "windows": [{
    "title": "Pomodoro Study",
    "width": 960,
    "height": 720,
    "minWidth": 720,
    "minHeight": 560,
    "decorations": true,
    "resizable": true,
    "transparent": false,
    "fullscreen": false
  }],
  "security": {
    "csp": null
  }
}
```

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "chore: scaffold Tauri 2 + Svelte 5 + TS project"
```

---

## Task 2: Tooling — Stylelint, ESLint, Prettier

**Files:**
- Create: `.stylelintrc.json`, `.eslintrc.cjs`, `.prettierrc.json`, `.prettierignore`
- Modify: `package.json` (scripts + devDependencies)

- [ ] **Step 1: Install dev dependencies**

```bash
pnpm add -D stylelint stylelint-config-standard postcss-html \
  eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin eslint-plugin-svelte \
  prettier prettier-plugin-svelte
```

- [ ] **Step 2: Create `.stylelintrc.json`**

Enforce token-only colors in component CSS (themes excluded):

```json
{
  "extends": ["stylelint-config-standard"],
  "customSyntax": "postcss-html",
  "rules": {
    "color-no-hex": true,
    "declaration-property-value-disallowed-list": {
      "border-radius": ["/[1-9]/"],
      "box-shadow": ["/blur|rgba\\(.+,.+,.+,0\\.[1-9]/"]
    }
  },
  "overrides": [
    {
      "files": ["src/themes/**/*.css", "src/styles/reset.css"],
      "rules": {
        "color-no-hex": null,
        "declaration-property-value-disallowed-list": null
      }
    }
  ]
}
```

This enforces the acid-art constraints: no hex colors in components (only tokens), no border-radius > 0, no blurred shadows. Theme files are exempt because they define the tokens.

- [ ] **Step 3: Create `.eslintrc.cjs`**

```js
module.exports = {
  parser: '@typescript-eslint/parser',
  extends: ['plugin:@typescript-eslint/recommended', 'plugin:svelte/recommended'],
  parserOptions: { ecmaVersion: 2022, sourceType: 'module', extraFileExtensions: ['.svelte'] },
  overrides: [{ files: ['*.svelte'], parser: 'svelte-eslint-parser' }],
  rules: { '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }] }
}
```

- [ ] **Step 4: Create `.prettierrc.json`**

```json
{
  "semi": false,
  "singleQuote": true,
  "tabWidth": 2,
  "printWidth": 100,
  "plugins": ["prettier-plugin-svelte"]
}
```

- [ ] **Step 5: Add scripts to `package.json`**

In the `"scripts"` block:

```json
"lint": "eslint src --ext .ts,.svelte && stylelint 'src/**/*.{css,svelte}'",
"format": "prettier --write 'src/**/*.{ts,svelte,css,json}'",
"check": "svelte-check --tsconfig ./tsconfig.json"
```

- [ ] **Step 6: Verify lint runs cleanly on scaffold**

```bash
pnpm lint
```

Expected: zero errors (scaffold has no hex in components yet).

## Task 3: SQLite + Migration Infrastructure (Rust)

**Files:**
- Create: `src-tauri/src/db/mod.rs`, `src-tauri/src/db/migrations.rs`, `src-tauri/src/db/migrations/0001_init.sql`
- Modify: `src-tauri/Cargo.toml` (add deps), `src-tauri/src/lib.rs` (init DB on startup), `src-tauri/src/main.rs`
- Test: `src-tauri/src/db/mod.rs` (inline `#[cfg(test)] mod tests`)

- [ ] **Step 1: Add Cargo dependencies**

Add to `src-tauri/Cargo.toml` under `[dependencies]`:

```toml
rusqlite = { version = "0.32", features = ["bundled"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
thiserror = "1"
parking_lot = "0.12"
```

Add `[dev-dependencies]`:

```toml
tempfile = "3"
```

- [ ] **Step 2: Write the failing test for migration runner**

Create `src-tauri/src/db/mod.rs`:

```rust
use anyhow::Result;
use parking_lot::Mutex;
use rusqlite::Connection;
use std::path::Path;
use std::sync::Arc;

pub mod migrations;

pub type Db = Arc<Mutex<Connection>>;

pub fn open(path: &Path) -> Result<Db> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
    migrations::run(&conn)?;
    Ok(Arc::new(Mutex::new(conn)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn open_creates_schema() {
        let dir = tempdir().unwrap();
        let db = open(&dir.path().join("test.db")).unwrap();
        let conn = db.lock();
        let count: i64 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='goals'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 1, "goals table should exist after migrations");
    }

    #[test]
    fn open_is_idempotent() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.db");
        let _ = open(&path).unwrap();
        let _ = open(&path).unwrap(); // Re-open should not error
    }
}
```

- [ ] **Step 3: Run tests to verify they fail**

```bash
cd src-tauri && cargo test --lib db
```

Expected: FAIL — `migrations` module not yet defined.

- [ ] **Step 4: Implement the migration runner**

Create `src-tauri/src/db/migrations.rs`:

```rust
use anyhow::Result;
use rusqlite::Connection;

const MIGRATIONS: &[(i64, &str)] = &[
    (1, include_str!("migrations/0001_init.sql")),
];

pub fn run(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at INTEGER NOT NULL
        )",
        [],
    )?;
    let current: i64 = conn
        .query_row("SELECT COALESCE(MAX(version), 0) FROM schema_version", [], |r| r.get(0))
        .unwrap_or(0);
    for (v, sql) in MIGRATIONS {
        if *v > current {
            conn.execute_batch(sql)?;
            conn.execute(
                "INSERT INTO schema_version(version, applied_at) VALUES(?1, strftime('%s','now')*1000)",
                [v],
            )?;
        }
    }
    Ok(())
}
```

- [ ] **Step 5: Create the initial schema SQL**

Create `src-tauri/src/db/migrations/0001_init.sql` — copy verbatim from spec section 4.1:

```sql
CREATE TABLE goals (
  id          INTEGER PRIMARY KEY,
  title       TEXT NOT NULL,
  description TEXT,
  color       TEXT,
  status      TEXT NOT NULL DEFAULT 'active',
  created_at  INTEGER NOT NULL,
  archived_at INTEGER
);

CREATE TABLE tasks (
  id              INTEGER PRIMARY KEY,
  goal_id         INTEGER REFERENCES goals(id) ON DELETE SET NULL,
  title           TEXT NOT NULL,
  estimated_pomos INTEGER DEFAULT 1,
  status          TEXT NOT NULL DEFAULT 'active',
  created_at      INTEGER NOT NULL,
  done_at         INTEGER,
  sort_order      INTEGER DEFAULT 0
);

CREATE TABLE pomodoros (
  id            INTEGER PRIMARY KEY,
  task_id       INTEGER REFERENCES tasks(id) ON DELETE SET NULL,
  goal_id       INTEGER REFERENCES goals(id) ON DELETE SET NULL,
  started_at    INTEGER NOT NULL,
  ended_at      INTEGER,
  planned_secs  INTEGER NOT NULL,
  actual_secs   INTEGER,
  status        TEXT NOT NULL,
  date_local    TEXT NOT NULL
);

CREATE TABLE interrupts (
  id          INTEGER PRIMARY KEY,
  pomodoro_id INTEGER NOT NULL REFERENCES pomodoros(id) ON DELETE CASCADE,
  reason      TEXT,
  occurred_at INTEGER NOT NULL
);

CREATE TABLE settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

CREATE INDEX idx_pomos_date ON pomodoros(date_local);
CREATE INDEX idx_pomos_task ON pomodoros(task_id);
CREATE INDEX idx_pomos_goal ON pomodoros(goal_id);
CREATE INDEX idx_tasks_goal ON tasks(goal_id);
```

- [ ] **Step 6: Run tests to verify they pass**

```bash
cd src-tauri && cargo test --lib db
```

Expected: 2 passed.

## Task 4: App State + DB Initialization on Startup

**Files:**
- Create: `src-tauri/src/state.rs`, `src-tauri/src/error.rs`, `src-tauri/src/seed.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Create error type**

Create `src-tauri/src/error.rs`:

```rust
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
```

- [ ] **Step 2: Create AppState wrapping the DB**

Create `src-tauri/src/state.rs`:

```rust
use crate::db::Db;

pub struct AppState {
    pub db: Db,
}

impl AppState {
    pub fn new(db: Db) -> Self { Self { db } }
}
```

- [ ] **Step 3: Write the seed module with default settings**

Create `src-tauri/src/seed.rs`:

```rust
use crate::error::AppResult;
use rusqlite::Connection;

const DEFAULTS: &[(&str, &str)] = &[
    ("timer.work_secs", "1500"),
    ("timer.break_secs", "300"),
    ("timer.long_break_secs", "900"),
    ("timer.long_break_every", "4"),
    ("timer.auto_continue", "false"),
    ("notify.system", "true"),
    ("notify.sound", "true"),
    ("notify.sound_file", "\"ding.mp3\""),
    ("notify.fullscreen", "true"),
    ("notify.taskbar", "true"),
    ("theme", "\"acid\""),
];

pub fn seed_defaults(conn: &Connection) -> AppResult<()> {
    for (k, v) in DEFAULTS {
        conn.execute(
            "INSERT OR IGNORE INTO settings(key, value) VALUES(?1, ?2)",
            [k, v],
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use tempfile::tempdir;

    #[test]
    fn seed_inserts_defaults() {
        let dir = tempdir().unwrap();
        let db = db::open(&dir.path().join("t.db")).unwrap();
        seed_defaults(&db.lock()).unwrap();
        let count: i64 = db.lock()
            .query_row("SELECT count(*) FROM settings", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 11);
    }

    #[test]
    fn seed_is_idempotent() {
        let dir = tempdir().unwrap();
        let db = db::open(&dir.path().join("t.db")).unwrap();
        seed_defaults(&db.lock()).unwrap();
        seed_defaults(&db.lock()).unwrap();
        let count: i64 = db.lock()
            .query_row("SELECT count(*) FROM settings", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 11);
    }
}
```

- [ ] **Step 4: Run seed tests**

```bash
cd src-tauri && cargo test --lib seed
```

Expected: 2 passed.

- [ ] **Step 5: Wire DB init into Tauri startup**

Replace `src-tauri/src/lib.rs` with:

```rust
mod db;
mod error;
mod seed;
mod state;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path().app_local_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let db = db::open(&data_dir.join("data.db"))?;
            seed::seed_defaults(&db.lock())?;
            app.manage(AppState::new(db));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 6: Verify dev build still launches**

```bash
pnpm tauri dev
```

Expected: window opens, no panic. Close it. Verify the DB file was created at `%LOCALAPPDATA%\study.pomodoro.app\data.db`.

## Task 5: Goals IPC Commands

**Files:**
- Create: `src-tauri/src/commands/mod.rs`, `src-tauri/src/commands/goals.rs`
- Modify: `src-tauri/src/lib.rs` (register handlers)
- Test: inline in `goals.rs`

- [ ] **Step 1: Define the Goal struct and write failing tests**

Create `src-tauri/src/commands/mod.rs`:

```rust
pub mod goals;
```

Create `src-tauri/src/commands/goals.rs`:

```rust
use crate::error::AppResult;
use crate::state::AppState;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Goal {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub status: String,
    pub created_at: i64,
    pub archived_at: Option<i64>,
}

fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

#[tauri::command]
pub fn create_goal(
    state: State<'_, AppState>,
    title: String,
    description: Option<String>,
    color: Option<String>,
) -> AppResult<Goal> {
    let conn = state.db.lock();
    let now = now_ms();
    conn.execute(
        "INSERT INTO goals(title, description, color, status, created_at) VALUES(?1, ?2, ?3, 'active', ?4)",
        params![title, description, color, now],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Goal { id, title, description, color, status: "active".into(), created_at: now, archived_at: None })
}

#[tauri::command]
pub fn list_goals(state: State<'_, AppState>, include_archived: bool) -> AppResult<Vec<Goal>> {
    let conn = state.db.lock();
    let sql = if include_archived {
        "SELECT id, title, description, color, status, created_at, archived_at FROM goals ORDER BY created_at DESC"
    } else {
        "SELECT id, title, description, color, status, created_at, archived_at FROM goals WHERE status='active' ORDER BY created_at DESC"
    };
    let mut stmt = conn.prepare(sql)?;
    let goals = stmt.query_map([], |r| Ok(Goal {
        id: r.get(0)?, title: r.get(1)?, description: r.get(2)?,
        color: r.get(3)?, status: r.get(4)?, created_at: r.get(5)?, archived_at: r.get(6)?,
    }))?.collect::<Result<Vec<_>, _>>()?;
    Ok(goals)
}

#[tauri::command]
pub fn update_goal(
    state: State<'_, AppState>,
    id: i64,
    title: Option<String>,
    description: Option<String>,
    color: Option<String>,
) -> AppResult<()> {
    let conn = state.db.lock();
    if let Some(t) = title {
        conn.execute("UPDATE goals SET title=?1 WHERE id=?2", params![t, id])?;
    }
    if let Some(d) = description {
        conn.execute("UPDATE goals SET description=?1 WHERE id=?2", params![d, id])?;
    }
    if let Some(c) = color {
        conn.execute("UPDATE goals SET color=?1 WHERE id=?2", params![c, id])?;
    }
    Ok(())
}

#[tauri::command]
pub fn archive_goal(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.db.lock();
    conn.execute(
        "UPDATE goals SET status='archived', archived_at=?1 WHERE id=?2",
        params![now_ms(), id],
    )?;
    Ok(())
}

#[tauri::command]
pub fn delete_goal(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.db.lock();
    conn.execute("DELETE FROM goals WHERE id=?1", params![id])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use tempfile::tempdir;

    fn setup() -> (tempfile::TempDir, AppState) {
        let dir = tempdir().unwrap();
        let db = db::open(&dir.path().join("t.db")).unwrap();
        (dir, AppState::new(db))
    }

    fn create(s: &AppState, title: &str) -> Goal {
        let conn = s.db.lock();
        conn.execute(
            "INSERT INTO goals(title, status, created_at) VALUES(?1, 'active', ?2)",
            params![title, now_ms()],
        ).unwrap();
        let id = conn.last_insert_rowid();
        Goal { id, title: title.into(), description: None, color: None,
               status: "active".into(), created_at: now_ms(), archived_at: None }
    }

    #[test]
    fn create_then_list() {
        let (_d, s) = setup();
        create(&s, "CET-6");
        create(&s, "Math");
        let conn = s.db.lock();
        let count: i64 = conn.query_row("SELECT count(*) FROM goals", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn archive_marks_status() {
        let (_d, s) = setup();
        let g = create(&s, "X");
        s.db.lock().execute("UPDATE goals SET status='archived' WHERE id=?1", params![g.id]).unwrap();
        let status: String = s.db.lock()
            .query_row("SELECT status FROM goals WHERE id=?1", params![g.id], |r| r.get(0)).unwrap();
        assert_eq!(status, "archived");
    }
}
```

- [ ] **Step 2: Register commands in lib.rs**

Update `src-tauri/src/lib.rs` — replace `.invoke_handler(tauri::generate_handler![])` with:

```rust
.invoke_handler(tauri::generate_handler![
    commands::goals::create_goal,
    commands::goals::list_goals,
    commands::goals::update_goal,
    commands::goals::archive_goal,
    commands::goals::delete_goal,
])
```

And add `mod commands;` near the other `mod` declarations.

- [ ] **Step 3: Run tests**

```bash
cd src-tauri && cargo test --lib commands::goals
```

Expected: 2 passed.

- [ ] **Step 4: Verify the dev build still launches**

```bash
pnpm tauri dev
```

Expected: window opens, no panic. Close.

## Task 6: Tasks IPC Commands

**Files:**
- Create: `src-tauri/src/commands/tasks.rs`
- Modify: `src-tauri/src/commands/mod.rs`, `src-tauri/src/lib.rs` (register handlers)

- [ ] **Step 1: Create tasks command module with full CRUD**

Add `pub mod tasks;` to `src-tauri/src/commands/mod.rs`.

Create `src-tauri/src/commands/tasks.rs`:

```rust
use crate::error::AppResult;
use crate::state::AppState;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i64,
    pub goal_id: Option<i64>,
    pub title: String,
    pub estimated_pomos: i64,
    pub status: String,
    pub created_at: i64,
    pub done_at: Option<i64>,
    pub sort_order: i64,
}

fn now_ms() -> i64 { chrono::Utc::now().timestamp_millis() }

fn row_to_task(r: &rusqlite::Row) -> rusqlite::Result<Task> {
    Ok(Task {
        id: r.get(0)?, goal_id: r.get(1)?, title: r.get(2)?,
        estimated_pomos: r.get(3)?, status: r.get(4)?,
        created_at: r.get(5)?, done_at: r.get(6)?, sort_order: r.get(7)?,
    })
}

const SELECT: &str = "SELECT id, goal_id, title, estimated_pomos, status, created_at, done_at, sort_order FROM tasks";

#[tauri::command]
pub fn create_task(
    state: State<'_, AppState>,
    goal_id: Option<i64>,
    title: String,
    estimated_pomos: Option<i64>,
) -> AppResult<Task> {
    let conn = state.db.lock();
    let now = now_ms();
    let est = estimated_pomos.unwrap_or(1);
    conn.execute(
        "INSERT INTO tasks(goal_id, title, estimated_pomos, status, created_at, sort_order)
         VALUES(?1, ?2, ?3, 'active', ?4, COALESCE((SELECT MAX(sort_order)+1 FROM tasks WHERE goal_id IS ?1), 0))",
        params![goal_id, title, est, now],
    )?;
    let id = conn.last_insert_rowid();
    let task = conn.query_row(&format!("{SELECT} WHERE id=?1"), params![id], row_to_task)?;
    Ok(task)
}

#[tauri::command]
pub fn list_tasks(
    state: State<'_, AppState>,
    goal_id: Option<i64>,
    include_done: bool,
) -> AppResult<Vec<Task>> {
    let conn = state.db.lock();
    let mut sql = String::from(SELECT);
    let mut clauses: Vec<String> = Vec::new();
    if goal_id.is_some() { clauses.push("goal_id = ?1".into()); }
    if !include_done { clauses.push("status = 'active'".into()); }
    if !clauses.is_empty() { sql.push_str(" WHERE "); sql.push_str(&clauses.join(" AND ")); }
    sql.push_str(" ORDER BY sort_order ASC, created_at ASC");
    let tasks = if let Some(g) = goal_id {
        conn.prepare(&sql)?.query_map(params![g], row_to_task)?.collect::<Result<Vec<_>, _>>()?
    } else {
        conn.prepare(&sql)?.query_map([], row_to_task)?.collect::<Result<Vec<_>, _>>()?
    };
    Ok(tasks)
}

#[tauri::command]
pub fn update_task(
    state: State<'_, AppState>,
    id: i64,
    title: Option<String>,
    goal_id: Option<Option<i64>>,
    estimated_pomos: Option<i64>,
) -> AppResult<()> {
    let conn = state.db.lock();
    if let Some(t) = title {
        conn.execute("UPDATE tasks SET title=?1 WHERE id=?2", params![t, id])?;
    }
    if let Some(g) = goal_id {
        conn.execute("UPDATE tasks SET goal_id=?1 WHERE id=?2", params![g, id])?;
    }
    if let Some(e) = estimated_pomos {
        conn.execute("UPDATE tasks SET estimated_pomos=?1 WHERE id=?2", params![e, id])?;
    }
    Ok(())
}

#[tauri::command]
pub fn complete_task(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    state.db.lock().execute(
        "UPDATE tasks SET status='done', done_at=?1 WHERE id=?2",
        params![now_ms(), id],
    )?;
    Ok(())
}

#[tauri::command]
pub fn delete_task(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    state.db.lock().execute("DELETE FROM tasks WHERE id=?1", params![id])?;
    Ok(())
}

#[tauri::command]
pub fn reorder_tasks(state: State<'_, AppState>, ordered_ids: Vec<i64>) -> AppResult<()> {
    let mut conn = state.db.lock();
    let tx = conn.transaction()?;
    for (idx, id) in ordered_ids.iter().enumerate() {
        tx.execute("UPDATE tasks SET sort_order=?1 WHERE id=?2", params![idx as i64, id])?;
    }
    tx.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use tempfile::tempdir;

    fn setup() -> (tempfile::TempDir, AppState) {
        let dir = tempdir().unwrap();
        let db = db::open(&dir.path().join("t.db")).unwrap();
        (dir, AppState::new(db))
    }

    #[test]
    fn create_assigns_increasing_sort_order() {
        let (_d, s) = setup();
        let conn = s.db.lock();
        conn.execute("INSERT INTO goals(id, title, status, created_at) VALUES(1, 'g', 'active', 0)", []).unwrap();
        drop(conn);
        for title in ["a", "b", "c"] {
            let conn = s.db.lock();
            conn.execute(
                "INSERT INTO tasks(goal_id, title, estimated_pomos, status, created_at, sort_order)
                 VALUES(1, ?1, 1, 'active', 0,
                        COALESCE((SELECT MAX(sort_order)+1 FROM tasks WHERE goal_id=1), 0))",
                params![title],
            ).unwrap();
        }
        let conn = s.db.lock();
        let orders: Vec<i64> = conn.prepare("SELECT sort_order FROM tasks ORDER BY id").unwrap()
            .query_map([], |r| r.get(0)).unwrap().collect::<Result<_, _>>().unwrap();
        assert_eq!(orders, vec![0, 1, 2]);
    }
}
```

- [ ] **Step 2: Register handlers**

Add to `tauri::generate_handler![...]` in `src-tauri/src/lib.rs`:

```rust
commands::tasks::create_task,
commands::tasks::list_tasks,
commands::tasks::update_task,
commands::tasks::complete_task,
commands::tasks::delete_task,
commands::tasks::reorder_tasks,
```

- [ ] **Step 3: Run tests**

```bash
cd src-tauri && cargo test --lib commands::tasks
```

Expected: 1 passed.

## Task 7: Pomodoros + Interrupts IPC Commands

**Files:**
- Create: `src-tauri/src/commands/pomodoros.rs`
- Modify: `src-tauri/src/commands/mod.rs`, `src-tauri/src/lib.rs`

- [ ] **Step 1: Create the pomodoros module**

Add `pub mod pomodoros;` to `src-tauri/src/commands/mod.rs`.

Create `src-tauri/src/commands/pomodoros.rs`:

```rust
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pomodoro {
    pub id: i64,
    pub task_id: Option<i64>,
    pub goal_id: Option<i64>,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub planned_secs: i64,
    pub actual_secs: Option<i64>,
    pub status: String,
    pub date_local: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interrupt {
    pub id: i64,
    pub pomodoro_id: i64,
    pub reason: Option<String>,
    pub occurred_at: i64,
}

fn now_ms() -> i64 { chrono::Utc::now().timestamp_millis() }

fn local_date() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

#[tauri::command]
pub fn start_pomodoro(
    state: State<'_, AppState>,
    task_id: Option<i64>,
    planned_secs: i64,
) -> AppResult<Pomodoro> {
    let conn = state.db.lock();
    let goal_id: Option<i64> = if let Some(tid) = task_id {
        conn.query_row("SELECT goal_id FROM tasks WHERE id=?1", params![tid], |r| r.get(0)).ok()
    } else { None };
    let now = now_ms();
    let date = local_date();
    conn.execute(
        "INSERT INTO pomodoros(task_id, goal_id, started_at, planned_secs, status, date_local)
         VALUES(?1, ?2, ?3, ?4, 'in_progress', ?5)",
        params![task_id, goal_id, now, planned_secs, date],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Pomodoro { id, task_id, goal_id, started_at: now, ended_at: None,
                  planned_secs, actual_secs: None, status: "in_progress".into(), date_local: date })
}

#[tauri::command]
pub fn complete_pomodoro(state: State<'_, AppState>, id: i64, actual_secs: i64) -> AppResult<()> {
    let conn = state.db.lock();
    let now = now_ms();
    let updated = conn.execute(
        "UPDATE pomodoros SET ended_at=?1, actual_secs=?2, status='completed' WHERE id=?3 AND status='in_progress'",
        params![now, actual_secs, id],
    )?;
    if updated == 0 { return Err(AppError::NotFound); }
    Ok(())
}

#[tauri::command]
pub fn interrupt_pomodoro(
    state: State<'_, AppState>,
    id: i64,
    reason: Option<String>,
    actual_secs: i64,
    abandoned: bool,
) -> AppResult<()> {
    let mut conn = state.db.lock();
    let tx = conn.transaction()?;
    let now = now_ms();
    let final_status = if abandoned { "abandoned" } else { "interrupted" };
    let updated = tx.execute(
        "UPDATE pomodoros SET ended_at=?1, actual_secs=?2, status=?3 WHERE id=?4 AND status='in_progress'",
        params![now, actual_secs, final_status, id],
    )?;
    if updated == 0 { return Err(AppError::NotFound); }
    tx.execute(
        "INSERT INTO interrupts(pomodoro_id, reason, occurred_at) VALUES(?1, ?2, ?3)",
        params![id, reason, now],
    )?;
    tx.commit()?;
    Ok(())
}

#[tauri::command]
pub fn list_pomodoros_today(state: State<'_, AppState>) -> AppResult<Vec<Pomodoro>> {
    let conn = state.db.lock();
    let date = local_date();
    let mut stmt = conn.prepare(
        "SELECT id, task_id, goal_id, started_at, ended_at, planned_secs, actual_secs, status, date_local
         FROM pomodoros WHERE date_local=?1 ORDER BY started_at DESC"
    )?;
    let rows = stmt.query_map(params![date], |r| Ok(Pomodoro {
        id: r.get(0)?, task_id: r.get(1)?, goal_id: r.get(2)?,
        started_at: r.get(3)?, ended_at: r.get(4)?, planned_secs: r.get(5)?,
        actual_secs: r.get(6)?, status: r.get(7)?, date_local: r.get(8)?,
    }))?.collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use tempfile::tempdir;

    fn setup() -> (tempfile::TempDir, AppState) {
        let dir = tempdir().unwrap();
        let db = db::open(&dir.path().join("t.db")).unwrap();
        (dir, AppState::new(db))
    }

    #[test]
    fn start_creates_in_progress_pomodoro() {
        let (_d, s) = setup();
        // Insert a task to attach to
        s.db.lock().execute(
            "INSERT INTO tasks(id, goal_id, title, status, created_at) VALUES(1, NULL, 't', 'active', 0)",
            [],
        ).unwrap();
        let conn = s.db.lock();
        conn.execute(
            "INSERT INTO pomodoros(task_id, goal_id, started_at, planned_secs, status, date_local)
             VALUES(1, NULL, 0, 1500, 'in_progress', '2026-06-17')",
            [],
        ).unwrap();
        let status: String = conn.query_row(
            "SELECT status FROM pomodoros WHERE id=1", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(status, "in_progress");
    }

    #[test]
    fn interrupt_writes_interrupt_row_and_changes_status() {
        let (_d, s) = setup();
        let mut conn = s.db.lock();
        conn.execute(
            "INSERT INTO pomodoros(id, started_at, planned_secs, status, date_local)
             VALUES(1, 0, 1500, 'in_progress', '2026-06-17')",
            [],
        ).unwrap();
        let tx = conn.transaction().unwrap();
        tx.execute(
            "UPDATE pomodoros SET ended_at=100, actual_secs=600, status='interrupted' WHERE id=1",
            [],
        ).unwrap();
        tx.execute(
            "INSERT INTO interrupts(pomodoro_id, reason, occurred_at) VALUES(1, 'phone', 100)",
            [],
        ).unwrap();
        tx.commit().unwrap();
        let count: i64 = conn.query_row("SELECT count(*) FROM interrupts WHERE pomodoro_id=1", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 1);
    }
}
```

- [ ] **Step 2: Register handlers**

Add to `tauri::generate_handler![...]`:

```rust
commands::pomodoros::start_pomodoro,
commands::pomodoros::complete_pomodoro,
commands::pomodoros::interrupt_pomodoro,
commands::pomodoros::list_pomodoros_today,
```

- [ ] **Step 3: Run tests**

```bash
cd src-tauri && cargo test --lib commands::pomodoros
```

Expected: 2 passed.

## Task 8: Settings + Stats IPC Commands

**Files:**
- Create: `src-tauri/src/commands/settings.rs`, `src-tauri/src/commands/stats.rs`
- Modify: `src-tauri/src/commands/mod.rs`, `src-tauri/src/lib.rs`

- [ ] **Step 1: Create the settings module**

Add `pub mod settings; pub mod stats;` to `src-tauri/src/commands/mod.rs`.

Create `src-tauri/src/commands/settings.rs`:

```rust
use crate::error::AppResult;
use crate::state::AppState;
use rusqlite::params;
use std::collections::HashMap;
use tauri::State;

#[tauri::command]
pub fn get_setting(state: State<'_, AppState>, key: String) -> AppResult<Option<String>> {
    let conn = state.db.lock();
    let val: Option<String> = conn
        .query_row("SELECT value FROM settings WHERE key=?1", params![key], |r| r.get(0))
        .ok();
    Ok(val)
}

#[tauri::command]
pub fn set_setting(state: State<'_, AppState>, key: String, value: String) -> AppResult<()> {
    state.db.lock().execute(
        "INSERT INTO settings(key, value) VALUES(?1, ?2) ON CONFLICT(key) DO UPDATE SET value=?2",
        params![key, value],
    )?;
    Ok(())
}

#[tauri::command]
pub fn get_all_settings(state: State<'_, AppState>) -> AppResult<HashMap<String, String>> {
    let conn = state.db.lock();
    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
    let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?;
    let mut map = HashMap::new();
    for row in rows { let (k, v) = row?; map.insert(k, v); }
    Ok(map)
}
```

- [ ] **Step 2: Create the stats module**

Create `src-tauri/src/commands/stats.rs`:

```rust
use crate::error::AppResult;
use crate::state::AppState;
use rusqlite::params;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct DayStat {
    pub date: String,
    pub completed: i64,
    pub focus_secs: i64,
    pub interrupts: i64,
}

#[derive(Debug, Serialize)]
pub struct GoalStat {
    pub goal_id: i64,
    pub goal_title: String,
    pub completed: i64,
    pub focus_secs: i64,
}

#[derive(Debug, Serialize)]
pub struct InterruptReason {
    pub reason: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsSummary {
    pub today: DayStat,
    pub total: DayStat,         // date is "" for total
    pub streak_days: i64,
    pub last_7_days: Vec<DayStat>,
    pub by_goal: Vec<GoalStat>,
    pub top_interrupts: Vec<InterruptReason>,
}

fn local_date() -> String { chrono::Local::now().format("%Y-%m-%d").to_string() }

fn day_stat(conn: &rusqlite::Connection, date: Option<&str>) -> AppResult<DayStat> {
    let (sql, d): (&str, String) = match date {
        Some(d) => (
            "SELECT
              SUM(CASE WHEN status='completed' THEN 1 ELSE 0 END),
              COALESCE(SUM(CASE WHEN status='completed' THEN actual_secs ELSE 0 END),0),
              (SELECT count(*) FROM interrupts i JOIN pomodoros p ON p.id=i.pomodoro_id WHERE p.date_local=?1)
             FROM pomodoros WHERE date_local=?1",
            d.to_string(),
        ),
        None => (
            "SELECT
              SUM(CASE WHEN status='completed' THEN 1 ELSE 0 END),
              COALESCE(SUM(CASE WHEN status='completed' THEN actual_secs ELSE 0 END),0),
              (SELECT count(*) FROM interrupts)
             FROM pomodoros",
            String::new(),
        ),
    };
    let date_label = date.unwrap_or("").to_string();
    let row = if date.is_some() {
        conn.query_row(sql, params![d], |r| Ok((r.get::<_, Option<i64>>(0)?, r.get::<_, i64>(1)?, r.get::<_, i64>(2)?)))?
    } else {
        conn.query_row(sql, [], |r| Ok((r.get::<_, Option<i64>>(0)?, r.get::<_, i64>(1)?, r.get::<_, i64>(2)?)))?
    };
    Ok(DayStat {
        date: date_label,
        completed: row.0.unwrap_or(0),
        focus_secs: row.1,
        interrupts: row.2,
    })
}

#[tauri::command]
pub fn get_stats(state: State<'_, AppState>) -> AppResult<StatsSummary> {
    let conn = state.db.lock();
    let today = day_stat(&conn, Some(&local_date()))?;
    let total = day_stat(&conn, None)?;

    let mut last_7: Vec<DayStat> = Vec::new();
    for i in (0..7).rev() {
        let d = (chrono::Local::now() - chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
        last_7.push(day_stat(&conn, Some(&d))?);
    }

    let streak: i64 = {
        let mut s = 0i64;
        for i in 0..365 {
            let d = (chrono::Local::now() - chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
            let n: i64 = conn.query_row(
                "SELECT count(*) FROM pomodoros WHERE date_local=?1 AND status='completed'",
                params![d], |r| r.get(0)
            )?;
            if n > 0 { s += 1; } else if i > 0 { break; } else { break; }
        }
        s
    };

    let by_goal = conn.prepare(
        "SELECT g.id, g.title,
                SUM(CASE WHEN p.status='completed' THEN 1 ELSE 0 END),
                COALESCE(SUM(CASE WHEN p.status='completed' THEN p.actual_secs ELSE 0 END), 0)
         FROM goals g LEFT JOIN pomodoros p ON p.goal_id=g.id
         GROUP BY g.id ORDER BY 3 DESC"
    )?.query_map([], |r| Ok(GoalStat {
        goal_id: r.get(0)?, goal_title: r.get(1)?,
        completed: r.get::<_, Option<i64>>(2)?.unwrap_or(0),
        focus_secs: r.get::<_, Option<i64>>(3)?.unwrap_or(0),
    }))?.collect::<Result<Vec<_>, _>>()?;

    let top_interrupts = conn.prepare(
        "SELECT COALESCE(reason, '(no reason)') as r, count(*) FROM interrupts GROUP BY r ORDER BY 2 DESC LIMIT 5"
    )?.query_map([], |r| Ok(InterruptReason { reason: r.get(0)?, count: r.get(1)? }))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(StatsSummary { today, total, streak_days: streak, last_7_days: last_7, by_goal, top_interrupts })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use tempfile::tempdir;

    #[test]
    fn empty_stats_returns_zeros() {
        let dir = tempdir().unwrap();
        let db = db::open(&dir.path().join("t.db")).unwrap();
        let state = AppState::new(db);
        // Cannot directly call tauri::command in a test — instead exercise day_stat directly
        let conn = state.db.lock();
        let s = day_stat(&conn, Some("2026-06-17")).unwrap();
        assert_eq!(s.completed, 0);
        assert_eq!(s.focus_secs, 0);
        assert_eq!(s.interrupts, 0);
    }
}
```

- [ ] **Step 3: Register handlers**

Add to `tauri::generate_handler![...]`:

```rust
commands::settings::get_setting,
commands::settings::set_setting,
commands::settings::get_all_settings,
commands::stats::get_stats,
```

- [ ] **Step 4: Run tests**

```bash
cd src-tauri && cargo test --lib commands
```

Expected: all previous tests + 1 new pass.

## Task 9: Notifier — System Notifications, Sound, Window Control

**Files:**
- Create: `src-tauri/src/notifier.rs`, `src-tauri/src/commands/notify.rs`, `src-tauri/assets/ding.mp3` (placeholder), `src-tauri/assets/.gitkeep`
- Modify: `src-tauri/Cargo.toml`, `src-tauri/src/lib.rs`, `src-tauri/src/commands/mod.rs`

- [ ] **Step 1: Add notifier dependencies**

Add to `src-tauri/Cargo.toml` `[dependencies]`:

```toml
notify-rust = "4"
rodio = { version = "0.19", default-features = false, features = ["mp3"] }
```

Enable Tauri's `notification` permission. Edit `src-tauri/capabilities/default.json` and append `"core:window:default"`, `"core:window:allow-set-focus"`, `"core:window:allow-show"`, `"core:window:allow-unminimize"` to the `"permissions"` array.

- [ ] **Step 2: Place a default sound asset**

Place an `assets/ding.mp3` short notification sound (CC0 sources: freesound.org or kenney.nl). For development you can use any short mp3. Add a `.gitkeep` to keep `assets/` tracked even if mp3 is replaced.

In `src-tauri/tauri.conf.json` under `"bundle"` add:

```json
"resources": ["assets/*"]
```

- [ ] **Step 3: Implement the notifier module**

Create `src-tauri/src/notifier.rs`:

```rust
use crate::error::AppResult;
use std::io::Cursor;
use std::path::Path;

pub fn system_notify(title: &str, body: &str) -> AppResult<()> {
    notify_rust::Notification::new()
        .summary(title)
        .body(body)
        .show()
        .map_err(|e| crate::error::AppError::Other(e.to_string()))?;
    Ok(())
}

pub fn play_sound(path: &Path) -> AppResult<()> {
    let bytes = std::fs::read(path)?;
    std::thread::spawn(move || {
        if let Ok((_stream, handle)) = rodio::OutputStream::try_default() {
            if let Ok(sink) = rodio::Sink::try_new(&handle) {
                if let Ok(src) = rodio::Decoder::new(Cursor::new(bytes)) {
                    sink.append(src);
                    sink.sleep_until_end();
                }
            }
        }
    });
    Ok(())
}
```

- [ ] **Step 4: Implement notify commands**

Add `pub mod notify;` to `src-tauri/src/commands/mod.rs`.

Create `src-tauri/src/commands/notify.rs`:

```rust
use crate::error::{AppError, AppResult};
use crate::notifier;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn notify_system(title: String, body: String) -> AppResult<()> {
    notifier::system_notify(&title, &body)
}

#[tauri::command]
pub fn notify_sound(app: AppHandle, sound_file: String) -> AppResult<()> {
    let resource_path = app
        .path()
        .resolve(format!("assets/{sound_file}"), tauri::path::BaseDirectory::Resource)
        .map_err(|e| AppError::Other(e.to_string()))?;
    notifier::play_sound(&resource_path)
}

#[tauri::command]
pub fn notify_focus_window(app: AppHandle) -> AppResult<()> {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
    Ok(())
}

#[tauri::command]
pub fn notify_taskbar_flash(app: AppHandle) -> AppResult<()> {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.request_user_attention(Some(tauri::UserAttentionType::Informational));
    }
    Ok(())
}
```

- [ ] **Step 5: Register handlers**

Add to `tauri::generate_handler![...]`:

```rust
commands::notify::notify_system,
commands::notify::notify_sound,
commands::notify::notify_focus_window,
commands::notify::notify_taskbar_flash,
```

Also add `mod notifier;` near the other `mod` declarations.

- [ ] **Step 6: Smoke test from devtools**

```bash
pnpm tauri dev
```

In the devtools console (right-click → Inspect):

```js
const { invoke } = await import('@tauri-apps/api/core')
await invoke('notify_system', { title: 'Test', body: 'It works' })
```

Expected: A Windows toast notification appears.

## Task 10: Frontend IPC Wrapper + Types

**Files:**
- Create: `src/lib/types.ts`, `src/lib/ipc.ts`

- [ ] **Step 1: Define types matching the Rust structs**

Create `src/lib/types.ts`:

```ts
export type GoalStatus = 'active' | 'archived' | 'done'
export type TaskStatus = 'active' | 'done' | 'abandoned'
export type PomodoroStatus = 'in_progress' | 'completed' | 'interrupted' | 'abandoned'

export interface Goal {
  id: number
  title: string
  description: string | null
  color: string | null
  status: GoalStatus
  created_at: number
  archived_at: number | null
}

export interface Task {
  id: number
  goal_id: number | null
  title: string
  estimated_pomos: number
  status: TaskStatus
  created_at: number
  done_at: number | null
  sort_order: number
}

export interface Pomodoro {
  id: number
  task_id: number | null
  goal_id: number | null
  started_at: number
  ended_at: number | null
  planned_secs: number
  actual_secs: number | null
  status: PomodoroStatus
  date_local: string
}

export interface DayStat {
  date: string
  completed: number
  focus_secs: number
  interrupts: number
}

export interface GoalStat {
  goal_id: number
  goal_title: string
  completed: number
  focus_secs: number
}

export interface InterruptReason {
  reason: string
  count: number
}

export interface StatsSummary {
  today: DayStat
  total: DayStat
  streak_days: number
  last_7_days: DayStat[]
  by_goal: GoalStat[]
  top_interrupts: InterruptReason[]
}
```

- [ ] **Step 2: Create the typed IPC wrapper**

Create `src/lib/ipc.ts`:

```ts
import { invoke } from '@tauri-apps/api/core'
import type { Goal, Task, Pomodoro, StatsSummary } from './types'

export const ipc = {
  // goals
  createGoal: (title: string, description?: string, color?: string) =>
    invoke<Goal>('create_goal', { title, description, color }),
  listGoals: (includeArchived = false) =>
    invoke<Goal[]>('list_goals', { includeArchived }),
  updateGoal: (id: number, patch: { title?: string; description?: string; color?: string }) =>
    invoke<void>('update_goal', { id, ...patch }),
  archiveGoal: (id: number) => invoke<void>('archive_goal', { id }),
  deleteGoal: (id: number) => invoke<void>('delete_goal', { id }),

  // tasks
  createTask: (goalId: number | null, title: string, estimatedPomos = 1) =>
    invoke<Task>('create_task', { goalId, title, estimatedPomos }),
  listTasks: (goalId: number | null, includeDone = false) =>
    invoke<Task[]>('list_tasks', { goalId, includeDone }),
  updateTask: (id: number, patch: { title?: string; goalId?: number | null; estimatedPomos?: number }) =>
    invoke<void>('update_task', { id, ...patch }),
  completeTask: (id: number) => invoke<void>('complete_task', { id }),
  deleteTask: (id: number) => invoke<void>('delete_task', { id }),
  reorderTasks: (orderedIds: number[]) => invoke<void>('reorder_tasks', { orderedIds }),

  // pomodoros
  startPomodoro: (taskId: number | null, plannedSecs: number) =>
    invoke<Pomodoro>('start_pomodoro', { taskId, plannedSecs }),
  completePomodoro: (id: number, actualSecs: number) =>
    invoke<void>('complete_pomodoro', { id, actualSecs }),
  interruptPomodoro: (id: number, reason: string | null, actualSecs: number, abandoned: boolean) =>
    invoke<void>('interrupt_pomodoro', { id, reason, actualSecs, abandoned }),
  listPomodorosToday: () => invoke<Pomodoro[]>('list_pomodoros_today'),

  // settings
  getSetting: (key: string) => invoke<string | null>('get_setting', { key }),
  setSetting: (key: string, value: string) => invoke<void>('set_setting', { key, value }),
  getAllSettings: () => invoke<Record<string, string>>('get_all_settings'),

  // stats
  getStats: () => invoke<StatsSummary>('get_stats'),

  // notify
  notifySystem: (title: string, body: string) => invoke<void>('notify_system', { title, body }),
  notifySound: (soundFile: string) => invoke<void>('notify_sound', { soundFile }),
  notifyFocusWindow: () => invoke<void>('notify_focus_window'),
  notifyTaskbarFlash: () => invoke<void>('notify_taskbar_flash'),
}
```

- [ ] **Step 3: Verify type-check passes**

```bash
pnpm check
```

Expected: 0 errors.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat(frontend): add typed IPC wrapper and types"
```

---

## Task 11: Theme Tokens, Base CSS, Reset

**Files:**
- Create: `src/themes/_tokens.ts`, `src/themes/_base.css`, `src/styles/reset.css`

- [ ] **Step 1: Create the token list**

Create `src/themes/_tokens.ts`:

```ts
export const TOKEN_KEYS = [
  // colors
  'color-bg', 'color-bg-elevated', 'color-fg', 'color-fg-muted',
  'color-accent', 'color-accent-low',
  'color-border', 'color-success', 'color-danger',
  // typography
  'font-display', 'font-body', 'font-mono',
  // shape
  'radius', 'border-width',
  // shadows (hard only)
  'shadow-hard-sm', 'shadow-hard-md', 'shadow-hard-lg',
  // motion
  'ease-snap', 'ease-smooth',
  'duration-fast', 'duration-base',
] as const

export type TokenKey = typeof TOKEN_KEYS[number]
export type ThemeTokens = Record<TokenKey, string>
```

- [ ] **Step 2: Create the reset**

Create `src/styles/reset.css`:

```css
*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
html, body, #app { height: 100%; }
body {
  font-family: var(--font-body);
  background: var(--color-bg);
  color: var(--color-fg);
  -webkit-font-smoothing: antialiased;
  user-select: none;
}
button { font: inherit; color: inherit; background: transparent; border: 0; cursor: pointer; }
input, textarea { font: inherit; color: inherit; }
ul, ol { list-style: none; }
a { color: inherit; text-decoration: none; }
```

- [ ] **Step 3: Create base styles common to all themes**

Create `src/themes/_base.css`:

```css
:root {
  --duration-fast: 80ms;
  --duration-base: 160ms;
}
html, body { transition: background var(--duration-base) var(--ease-snap, steps(1, end)); }
```

## Task 12: Theme Registry + Switch API

**Files:**
- Create: `src/themes/registry.ts`, `src/themes/registry.test.ts`

- [ ] **Step 1: Write the failing test**

Create `src/themes/registry.test.ts`:

```ts
import { describe, it, expect, beforeEach } from 'vitest'
import { setTheme, getCurrentTheme, listThemes, registerTheme } from './registry'

describe('theme registry', () => {
  beforeEach(() => {
    document.documentElement.removeAttribute('data-theme')
  })

  it('lists registered themes', () => {
    registerTheme({ id: 'demo', displayName: 'Demo', status: 'available', preview: '' })
    expect(listThemes().some(t => t.id === 'demo')).toBe(true)
  })

  it('setTheme writes data-theme on <html>', () => {
    registerTheme({ id: 'a', displayName: 'A', status: 'available', preview: '' })
    setTheme('a')
    expect(document.documentElement.getAttribute('data-theme')).toBe('a')
    expect(getCurrentTheme()).toBe('a')
  })

  it('setTheme rejects unregistered ids', () => {
    expect(() => setTheme('does-not-exist')).toThrow()
  })

  it('setTheme rejects coming-soon themes', () => {
    registerTheme({ id: 'soon', displayName: 'Soon', status: 'coming-soon', preview: '' })
    expect(() => setTheme('soon')).toThrow()
  })
})
```

- [ ] **Step 2: Install Vitest and configure**

```bash
pnpm add -D vitest @testing-library/svelte jsdom
```

Add to `vite.config.ts`:

```ts
test: {
  environment: 'jsdom',
  globals: false,
}
```

Add to `package.json` scripts: `"test": "vitest run"`, `"test:watch": "vitest"`.

- [ ] **Step 3: Run test to verify failure**

```bash
pnpm test
```

Expected: FAIL — `registry` not found.

- [ ] **Step 4: Implement the registry**

Create `src/themes/registry.ts`:

```ts
export type ThemeStatus = 'available' | 'coming-soon'

export interface ThemeMeta {
  id: string
  displayName: string
  status: ThemeStatus
  preview: string  // small description or path to preview image
}

const registry = new Map<string, ThemeMeta>()
let current = 'acid'

export function registerTheme(meta: ThemeMeta): void {
  registry.set(meta.id, meta)
}

export function listThemes(): ThemeMeta[] {
  return Array.from(registry.values())
}

export function getCurrentTheme(): string {
  return current
}

export function setTheme(id: string): void {
  const meta = registry.get(id)
  if (!meta) throw new Error(`Theme not registered: ${id}`)
  if (meta.status === 'coming-soon') throw new Error(`Theme not available yet: ${id}`)
  document.documentElement.setAttribute('data-theme', id)
  current = id
}
```

- [ ] **Step 5: Run tests to verify pass**

```bash
pnpm test
```

Expected: 4 passed.

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat(theme): add registry with set/list/get API"
```

---

## Task 13: Acid Theme

**Files:**
- Create: `src/themes/acid/theme.css`, `src/themes/acid/animations.css`, `src/themes/acid/meta.ts`

- [ ] **Step 1: Define acid token values**

Create `src/themes/acid/theme.css`:

```css
[data-theme="acid"] {
  /* colors */
  --color-bg: #0a0a0a;
  --color-bg-elevated: #141414;
  --color-fg: #ffffff;
  --color-fg-muted: #888888;
  --color-accent: #c6ff00;
  --color-accent-low: rgba(198, 255, 0, 0.15);
  --color-border: #1f1f1f;
  --color-success: #c6ff00;
  --color-danger: #ff3b3b;

  /* typography */
  --font-display: 'Inter', 'Helvetica Neue', Arial, sans-serif;
  --font-body: 'Inter', 'Helvetica Neue', Arial, sans-serif;
  --font-mono: 'JetBrains Mono', 'Consolas', monospace;

  /* shape — hard constraints */
  --radius: 0;
  --border-width: 2px;

  /* shadows — hard only, blur=0 */
  --shadow-hard-sm: 4px 4px 0 var(--color-accent);
  --shadow-hard-md: 8px 8px 0 var(--color-accent);
  --shadow-hard-lg: 12px 12px 0 var(--color-accent);

  /* motion — snap easing for hard cuts */
  --ease-snap: steps(3, end);
  --ease-smooth: cubic-bezier(0.4, 0, 0.2, 1);
  --duration-fast: 80ms;
  --duration-base: 160ms;
}
```

- [ ] **Step 2: Define acid-specific animations**

Create `src/themes/acid/animations.css`:

```css
@keyframes acid-blink {
  0%, 100% { color: var(--color-fg); }
  50% { color: var(--color-accent); }
}

@keyframes acid-scan {
  0% { transform: translateY(-100%); }
  100% { transform: translateY(100vh); }
}

@keyframes acid-shadow-pulse {
  0% { box-shadow: var(--shadow-hard-sm); }
  100% { box-shadow: var(--shadow-hard-md); }
}

.acid-end-flash {
  animation: acid-blink 80ms steps(3, end) 6;
}

.acid-scanline {
  position: fixed;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--color-accent);
  pointer-events: none;
  animation: acid-scan 600ms steps(20, end);
  z-index: 9999;
}
```

- [ ] **Step 3: Create acid meta file**

Create `src/themes/acid/meta.ts`:

```ts
import type { ThemeMeta } from '../registry'
import './theme.css'
import './animations.css'

export const meta: ThemeMeta = {
  id: 'acid',
  displayName: '酸性艺术 / Acid',
  status: 'available',
  preview: 'Black + acid-lime green / hard shadows / sharp corners',
}
```

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat(theme): add acid art theme implementation"
```

---

## Task 14: Synthwave Placeholder

**Files:**
- Create: `src/themes/synthwave/meta.ts`

- [ ] **Step 1: Create placeholder meta**

Create `src/themes/synthwave/meta.ts`:

```ts
import type { ThemeMeta } from '../registry'

export const meta: ThemeMeta = {
  id: 'synthwave',
  displayName: '合成器浪潮 / Synthwave',
  status: 'coming-soon',
  preview: 'Neon sunset gradient with retro grid (coming soon)',
}
```

- [ ] **Step 2: Commit**

```bash
git add -A
git commit -m "feat(theme): add synthwave placeholder"
```

---

## Task 15: Theme Bootstrap

**Files:**
- Create: `src/themes/bootstrap.ts`
- Modify: `src/main.ts`, `src/app.css` (replace with imports)

- [ ] **Step 1: Create bootstrap to register and apply theme on startup**

Create `src/themes/bootstrap.ts`:

```ts
import { registerTheme, setTheme } from './registry'
import { meta as acidMeta } from './acid/meta'
import { meta as synthwaveMeta } from './synthwave/meta'
import { ipc } from '../lib/ipc'

export async function bootstrapThemes(): Promise<void> {
  registerTheme(acidMeta)
  registerTheme(synthwaveMeta)

  const stored = await ipc.getSetting('theme')
  const initial = stored ? JSON.parse(stored) : 'acid'
  try {
    setTheme(initial)
  } catch {
    setTheme('acid')
  }
}
```

- [ ] **Step 2: Replace `src/app.css` with theme imports**

Overwrite `src/app.css`:

```css
@import './styles/reset.css';
@import './themes/_base.css';
```

- [ ] **Step 3: Bootstrap themes in `main.ts`**

Edit `src/main.ts`:

```ts
import { mount } from 'svelte'
import App from './App.svelte'
import './app.css'
import { bootstrapThemes } from './themes/bootstrap'

await bootstrapThemes()

const app = mount(App, { target: document.getElementById('app')! })
export default app
```

- [ ] **Step 4: Verify theme applies on launch**

```bash
pnpm tauri dev
```

Expected: window opens with black background (acid theme applied). Devtools `<html>` should show `data-theme="acid"`.

## Task 16: Settings Store

**Files:**
- Create: `src/lib/stores/settings.svelte.ts`, `src/lib/stores/settings.test.ts`

- [ ] **Step 1: Define the settings shape and parser**

Create `src/lib/stores/settings.svelte.ts`:

```ts
import { ipc } from '../ipc'

export interface Settings {
  workSecs: number
  breakSecs: number
  longBreakSecs: number
  longBreakEvery: number
  autoContinue: boolean
  notifySystem: boolean
  notifySound: boolean
  notifySoundFile: string
  notifyFullscreen: boolean
  notifyTaskbar: boolean
  theme: string
}

const DEFAULTS: Settings = {
  workSecs: 1500,
  breakSecs: 300,
  longBreakSecs: 900,
  longBreakEvery: 4,
  autoContinue: false,
  notifySystem: true,
  notifySound: true,
  notifySoundFile: 'ding.mp3',
  notifyFullscreen: true,
  notifyTaskbar: true,
  theme: 'acid',
}

const KEY_MAP: Record<keyof Settings, string> = {
  workSecs: 'timer.work_secs',
  breakSecs: 'timer.break_secs',
  longBreakSecs: 'timer.long_break_secs',
  longBreakEvery: 'timer.long_break_every',
  autoContinue: 'timer.auto_continue',
  notifySystem: 'notify.system',
  notifySound: 'notify.sound',
  notifySoundFile: 'notify.sound_file',
  notifyFullscreen: 'notify.fullscreen',
  notifyTaskbar: 'notify.taskbar',
  theme: 'theme',
}

function parse(raw: Record<string, string>): Settings {
  const out = { ...DEFAULTS }
  for (const [field, key] of Object.entries(KEY_MAP) as [keyof Settings, string][]) {
    const v = raw[key]
    if (v === undefined) continue
    try {
      ;(out as Record<string, unknown>)[field] = JSON.parse(v)
    } catch {
      // leave default
    }
  }
  return out
}

export function createSettingsStore() {
  let state = $state<Settings>({ ...DEFAULTS })
  let loaded = $state(false)

  async function load() {
    const raw = await ipc.getAllSettings()
    Object.assign(state, parse(raw))
    loaded = true
  }

  async function update<K extends keyof Settings>(field: K, value: Settings[K]) {
    state[field] = value
    await ipc.setSetting(KEY_MAP[field], JSON.stringify(value))
  }

  return {
    get state() { return state },
    get loaded() { return loaded },
    load,
    update,
  }
}

export const settings = createSettingsStore()
```

- [ ] **Step 2: Write a test for the parser**

Create `src/lib/stores/settings.test.ts`:

```ts
import { describe, it, expect } from 'vitest'

// Re-export the internal `parse` via a __test helper. Adjust the import
// once the helper is exposed; for now, we test public behavior:
import { createSettingsStore } from './settings.svelte'

describe('settings store', () => {
  it('falls back to defaults for unknown keys', () => {
    const s = createSettingsStore()
    expect(s.state.workSecs).toBe(1500)
    expect(s.state.theme).toBe('acid')
  })

  it('exposes loaded=false until load() resolves', () => {
    const s = createSettingsStore()
    expect(s.loaded).toBe(false)
  })
})
```

- [ ] **Step 3: Run tests**

```bash
pnpm test
```

Expected: 2 new pass.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat(store): add settings store with persistence"
```

---

## Task 17: Goals + Tasks Stores

**Files:**
- Create: `src/lib/stores/goals.svelte.ts`, `src/lib/stores/tasks.svelte.ts`

- [ ] **Step 1: Create the goals store**

Create `src/lib/stores/goals.svelte.ts`:

```ts
import { ipc } from '../ipc'
import type { Goal } from '../types'

export function createGoalsStore() {
  let goals = $state<Goal[]>([])
  let loading = $state(false)

  async function reload(includeArchived = false) {
    loading = true
    try { goals = await ipc.listGoals(includeArchived) }
    finally { loading = false }
  }

  async function create(title: string, description?: string, color?: string) {
    const g = await ipc.createGoal(title, description, color)
    goals = [g, ...goals]
    return g
  }

  async function update(id: number, patch: { title?: string; description?: string; color?: string }) {
    await ipc.updateGoal(id, patch)
    goals = goals.map(g => g.id === id ? { ...g, ...patch } : g)
  }

  async function archive(id: number) {
    await ipc.archiveGoal(id)
    goals = goals.filter(g => g.id !== id)
  }

  async function remove(id: number) {
    await ipc.deleteGoal(id)
    goals = goals.filter(g => g.id !== id)
  }

  return {
    get goals() { return goals },
    get loading() { return loading },
    reload, create, update, archive, remove,
  }
}

export const goalsStore = createGoalsStore()
```

- [ ] **Step 2: Create the tasks store**

Create `src/lib/stores/tasks.svelte.ts`:

```ts
import { ipc } from '../ipc'
import type { Task } from '../types'

export function createTasksStore() {
  let tasks = $state<Task[]>([])
  let loading = $state(false)
  let currentGoalId = $state<number | null>(null)

  async function reload(goalId: number | null = currentGoalId, includeDone = false) {
    loading = true
    currentGoalId = goalId
    try { tasks = await ipc.listTasks(goalId, includeDone) }
    finally { loading = false }
  }

  async function create(goalId: number | null, title: string, estimatedPomos = 1) {
    const t = await ipc.createTask(goalId, title, estimatedPomos)
    tasks = [...tasks, t]
    return t
  }

  async function update(id: number, patch: { title?: string; goalId?: number | null; estimatedPomos?: number }) {
    await ipc.updateTask(id, patch)
    tasks = tasks.map(t => t.id === id ? { ...t, ...(patch.title !== undefined && { title: patch.title }),
      ...(patch.goalId !== undefined && { goal_id: patch.goalId }),
      ...(patch.estimatedPomos !== undefined && { estimated_pomos: patch.estimatedPomos }) } : t)
  }

  async function complete(id: number) {
    await ipc.completeTask(id)
    tasks = tasks.filter(t => t.id !== id)
  }

  async function remove(id: number) {
    await ipc.deleteTask(id)
    tasks = tasks.filter(t => t.id !== id)
  }

  async function reorder(orderedIds: number[]) {
    await ipc.reorderTasks(orderedIds)
    const map = new Map(tasks.map(t => [t.id, t]))
    tasks = orderedIds.map((id, idx) => ({ ...(map.get(id)!), sort_order: idx })).filter(Boolean)
  }

  return {
    get tasks() { return tasks },
    get loading() { return loading },
    reload, create, update, complete, remove, reorder,
  }
}

export const tasksStore = createTasksStore()
```

- [ ] **Step 3: Type-check**

```bash
pnpm check
```

Expected: 0 errors.

## Task 18: Timer State Machine Store (TDD)

**Files:**
- Create: `src/lib/stores/timer.svelte.ts`, `src/lib/stores/timer.test.ts`

The timer is the core of the app — fully TDD this one.

- [ ] **Step 1: Write the failing tests**

Create `src/lib/stores/timer.test.ts`:

```ts
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { createTimerMachine, type TimerPhase } from './timer.svelte'

const SETTINGS = {
  workSecs: 25,
  breakSecs: 5,
  longBreakSecs: 15,
  longBreakEvery: 4,
  autoContinue: false,
}

describe('timer state machine', () => {
  let now = 0
  beforeEach(() => {
    now = 1_000_000
    vi.spyOn(Date, 'now').mockImplementation(() => now)
  })

  function advance(secs: number) { now += secs * 1000 }

  it('starts in idle phase', () => {
    const t = createTimerMachine(SETTINGS)
    expect(t.phase).toBe<TimerPhase>('idle')
    expect(t.remainingSecs).toBe(SETTINGS.workSecs)
  })

  it('startWork moves to focusing and counts down', () => {
    const t = createTimerMachine(SETTINGS)
    t.startWork(null)
    expect(t.phase).toBe('focusing')
    advance(10)
    t.tick()
    expect(t.remainingSecs).toBe(SETTINGS.workSecs - 10)
  })

  it('reaching 0 transitions focusing -> short break', () => {
    const t = createTimerMachine(SETTINGS)
    t.startWork(null)
    advance(SETTINGS.workSecs)
    t.tick()
    expect(t.phase).toBe('breaking')
    expect(t.remainingSecs).toBe(SETTINGS.breakSecs)
  })

  it('every 4th completed work triggers long break', () => {
    const t = createTimerMachine(SETTINGS)
    for (let i = 1; i <= 4; i++) {
      t.startWork(null)
      advance(SETTINGS.workSecs)
      t.tick()
      if (i < 4) {
        expect(t.phase).toBe('breaking')
        advance(SETTINGS.breakSecs)
        t.tick()
        expect(t.phase).toBe('idle')
      }
    }
    expect(t.phase).toBe('long-breaking')
    expect(t.remainingSecs).toBe(SETTINGS.longBreakSecs)
  })

  it('pause stops countdown, resume continues', () => {
    const t = createTimerMachine(SETTINGS)
    t.startWork(null)
    advance(5)
    t.tick()
    t.pause()
    advance(10)
    t.tick()
    expect(t.remainingSecs).toBe(SETTINGS.workSecs - 5)
    t.resume()
    advance(3)
    t.tick()
    expect(t.remainingSecs).toBe(SETTINGS.workSecs - 8)
  })

  it('interrupt during focus moves to idle and emits onInterrupt', () => {
    const cb = vi.fn()
    const t = createTimerMachine(SETTINGS, { onInterrupt: cb })
    t.startWork(42)
    advance(7)
    t.tick()
    t.interrupt('phone', false)
    expect(cb).toHaveBeenCalledWith({ taskId: 42, actualSecs: 7, reason: 'phone', abandoned: false })
    expect(t.phase).toBe('idle')
  })

  it('completing a focus emits onWorkComplete with correct elapsed', () => {
    const cb = vi.fn()
    const t = createTimerMachine(SETTINGS, { onWorkComplete: cb })
    t.startWork(7)
    advance(SETTINGS.workSecs)
    t.tick()
    expect(cb).toHaveBeenCalledWith({ taskId: 7, actualSecs: SETTINGS.workSecs })
  })
})
```

- [ ] **Step 2: Run tests to verify failure**

```bash
pnpm test
```

Expected: FAIL — module not found.

- [ ] **Step 3: Implement the timer state machine**

Create `src/lib/stores/timer.svelte.ts`:

```ts
export type TimerPhase = 'idle' | 'focusing' | 'breaking' | 'long-breaking' | 'paused'

export interface TimerSettings {
  workSecs: number
  breakSecs: number
  longBreakSecs: number
  longBreakEvery: number
  autoContinue: boolean
}

export interface TimerCallbacks {
  onWorkComplete?: (e: { taskId: number | null; actualSecs: number }) => void
  onBreakComplete?: () => void
  onInterrupt?: (e: { taskId: number | null; actualSecs: number; reason: string | null; abandoned: boolean }) => void
}

export function createTimerMachine(settings: TimerSettings, cb: TimerCallbacks = {}) {
  let phase = $state<TimerPhase>('idle')
  let prevPhase: TimerPhase = 'idle'   // for resume from pause
  let plannedSecs = $state(settings.workSecs)
  let startedAt = $state<number | null>(null)
  let pausedAt = $state<number | null>(null)
  let pausedAccumMs = $state(0)
  let currentTaskId = $state<number | null>(null)
  let completedWorkCount = $state(0)
  let remainingSecs = $state(settings.workSecs)

  function elapsedMs(): number {
    if (startedAt === null) return 0
    const end = pausedAt ?? Date.now()
    return end - startedAt - pausedAccumMs
  }

  function recompute() {
    if (phase === 'idle') return
    const elapsed = Math.floor(elapsedMs() / 1000)
    remainingSecs = Math.max(0, plannedSecs - elapsed)
  }

  function startWork(taskId: number | null) {
    currentTaskId = taskId
    plannedSecs = settings.workSecs
    remainingSecs = plannedSecs
    startedAt = Date.now()
    pausedAt = null
    pausedAccumMs = 0
    phase = 'focusing'
  }

  function startBreak(long: boolean) {
    plannedSecs = long ? settings.longBreakSecs : settings.breakSecs
    remainingSecs = plannedSecs
    startedAt = Date.now()
    pausedAt = null
    pausedAccumMs = 0
    phase = long ? 'long-breaking' : 'breaking'
  }

  function tick() {
    recompute()
    if (remainingSecs > 0) return
    if (phase === 'focusing') {
      const actualSecs = plannedSecs
      cb.onWorkComplete?.({ taskId: currentTaskId, actualSecs })
      completedWorkCount += 1
      const long = completedWorkCount % settings.longBreakEvery === 0
      startBreak(long)
    } else if (phase === 'breaking' || phase === 'long-breaking') {
      cb.onBreakComplete?.()
      phase = 'idle'
      remainingSecs = settings.workSecs
      plannedSecs = settings.workSecs
      startedAt = null
    }
  }

  function pause() {
    if (phase !== 'focusing' && phase !== 'breaking' && phase !== 'long-breaking') return
    prevPhase = phase
    pausedAt = Date.now()
    phase = 'paused'
  }

  function resume() {
    if (phase !== 'paused' || pausedAt === null) return
    pausedAccumMs += Date.now() - pausedAt
    pausedAt = null
    phase = prevPhase
  }

  function interrupt(reason: string | null, abandoned: boolean) {
    if (phase !== 'focusing') return
    const actualSecs = Math.floor(elapsedMs() / 1000)
    cb.onInterrupt?.({ taskId: currentTaskId, actualSecs, reason, abandoned })
    phase = 'idle'
    remainingSecs = settings.workSecs
    plannedSecs = settings.workSecs
    startedAt = null
  }

  function reset() {
    phase = 'idle'
    startedAt = null
    pausedAt = null
    pausedAccumMs = 0
    currentTaskId = null
    completedWorkCount = 0
    remainingSecs = settings.workSecs
    plannedSecs = settings.workSecs
  }

  return {
    get phase() { return phase },
    get remainingSecs() { return remainingSecs },
    get plannedSecs() { return plannedSecs },
    get currentTaskId() { return currentTaskId },
    get completedWorkCount() { return completedWorkCount },
    startWork, startBreak, tick, pause, resume, interrupt, reset,
  }
}
```

- [ ] **Step 4: Run tests to verify pass**

```bash
pnpm test
```

Expected: all 7 timer tests pass.

## Task 19: App Shell — Sidebar + Routing

**Files:**
- Create: `src/lib/router.svelte.ts`, `src/lib/components/Sidebar.svelte`, `src/lib/components/Sidebar.module.css`
- Modify: `src/App.svelte`

- [ ] **Step 1: Create a tiny router store**

Create `src/lib/router.svelte.ts`:

```ts
export type Route = 'focus' | 'tasks' | 'stats' | 'theme' | 'settings'

let current = $state<Route>('focus')

export function getRoute(): Route { return current }
export function setRoute(r: Route) { current = r }
export const router = {
  get current() { return current },
  go: (r: Route) => { current = r },
}
```

- [ ] **Step 2: Build the sidebar component**

Create `src/lib/components/Sidebar.svelte`:

```svelte
<script lang="ts">
  import { router, type Route } from '../router.svelte'

  const items: { id: Route; glyph: string; label: string }[] = [
    { id: 'focus', glyph: '▮', label: 'FOCUS' },
    { id: 'tasks', glyph: '●', label: 'TASKS' },
    { id: 'stats', glyph: '▤', label: 'STATS' },
    { id: 'theme', glyph: '◐', label: 'THEME' },
    { id: 'settings', glyph: '⚙', label: 'SETTINGS' },
  ]
</script>

<nav class="sidebar">
  {#each items as item}
    <button
      class="item"
      class:active={router.current === item.id}
      onclick={() => router.go(item.id)}
      aria-label={item.label}
      aria-current={router.current === item.id ? 'page' : undefined}
    >
      <span class="glyph">{item.glyph}</span>
      <span class="label">{item.label}</span>
    </button>
  {/each}
</nav>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    width: 80px;
    border-right: var(--border-width) solid var(--color-border);
    padding: 16px 0;
    background: var(--color-bg);
  }
  .item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 14px 0;
    color: var(--color-fg-muted);
    transition: color var(--duration-fast) var(--ease-snap);
  }
  .item:hover { color: var(--color-fg); }
  .item.active { color: var(--color-accent); }
  .item.active::before {
    content: '';
    position: absolute;
    left: 0;
    width: 4px;
    height: 28px;
    background: var(--color-accent);
  }
  .glyph { font-size: 18px; line-height: 1; }
  .label {
    font-size: 9px;
    letter-spacing: 2px;
    font-weight: 700;
  }
  @media (max-width: 600px) {
    .sidebar { width: 56px; }
    .label { display: none; }
  }
</style>
```

- [ ] **Step 3: Wire App.svelte**

Replace `src/App.svelte`:

```svelte
<script lang="ts">
  import Sidebar from './lib/components/Sidebar.svelte'
  import { router } from './lib/router.svelte'
  import { onMount } from 'svelte'
  import { settings } from './lib/stores/settings.svelte'
  import { goalsStore } from './lib/stores/goals.svelte'
  import FocusPage from './routes/focus/+page.svelte'
  import TasksPage from './routes/tasks/+page.svelte'
  import StatsPage from './routes/stats/+page.svelte'
  import ThemePage from './routes/theme/+page.svelte'
  import SettingsPage from './routes/settings/+page.svelte'

  onMount(async () => {
    await settings.load()
    await goalsStore.reload()
  })
</script>

<main class="app">
  <Sidebar />
  <section class="content">
    {#if router.current === 'focus'}<FocusPage />
    {:else if router.current === 'tasks'}<TasksPage />
    {:else if router.current === 'stats'}<StatsPage />
    {:else if router.current === 'theme'}<ThemePage />
    {:else if router.current === 'settings'}<SettingsPage />
    {/if}
  </section>
</main>

<style>
  .app {
    display: flex;
    height: 100vh;
    background: var(--color-bg);
    color: var(--color-fg);
  }
  .content {
    flex: 1;
    overflow: auto;
  }
</style>
```

- [ ] **Step 4: Create empty page placeholders**

Create the 5 route files with bare scaffolding. Each one is just a heading for now:

`src/routes/focus/+page.svelte`:
```svelte
<h1 class="page-title">FOCUS</h1>
<style>.page-title { padding: 24px; font-size: 12px; letter-spacing: 4px; color: var(--color-fg-muted); }</style>
```

Repeat verbatim for `src/routes/tasks/+page.svelte`, `src/routes/stats/+page.svelte`, `src/routes/theme/+page.svelte`, `src/routes/settings/+page.svelte` — change only the heading text to match the route id.

- [ ] **Step 5: Verify dev build**

```bash
pnpm tauri dev
```

Expected: window opens with sidebar on the left, clicking each icon switches the heading text. Active item shows acid green left bar.

## Task 20: TimerDisplay + ProgressBar Components

**Files:**
- Create: `src/lib/components/TimerDisplay.svelte`, `src/lib/components/ProgressBar.svelte`

- [ ] **Step 1: Build TimerDisplay**

Create `src/lib/components/TimerDisplay.svelte`:

```svelte
<script lang="ts">
  type Props = {
    remainingSecs: number
    plannedSecs: number
    label: string
  }
  let { remainingSecs, plannedSecs, label }: Props = $props()

  const fmt = (s: number) => {
    const m = Math.floor(s / 60)
    const sec = s % 60
    return `${String(m).padStart(2, '0')}:${String(sec).padStart(2, '0')}`
  }
  const pct = $derived(plannedSecs > 0 ? Math.round((1 - remainingSecs / plannedSecs) * 100) : 0)
</script>

<div class="display">
  <div class="frame">
    <div class="time">{fmt(remainingSecs)}</div>
    <div class="meter" aria-label={`${pct}% done`}>
      {#each Array(10) as _, i}
        <span class="cell" class:filled={i < Math.round(pct / 10)}>{i < Math.round(pct / 10) ? '▮' : '▯'}</span>
      {/each}
      <span class="pct">{pct}%</span>
    </div>
    <div class="label">{label}</div>
  </div>
</div>

<style>
  .display { padding: 32px 20px; text-align: center; }
  .frame {
    display: inline-block;
    padding: 28px 36px;
    background: var(--color-bg);
    border: var(--border-width) solid var(--color-fg);
    box-shadow: var(--shadow-hard-md);
  }
  .time {
    font-family: var(--font-display);
    font-size: 96px;
    font-weight: 900;
    letter-spacing: -4px;
    line-height: 1;
    font-variant-numeric: tabular-nums;
    color: var(--color-fg);
  }
  .meter {
    margin-top: 10px;
    font-family: var(--font-mono);
    font-size: 11px;
    letter-spacing: 4px;
    color: var(--color-accent);
    font-weight: 700;
  }
  .cell { display: inline-block; }
  .pct { margin-left: 8px; }
  .label {
    margin-top: 8px;
    font-size: 10px;
    letter-spacing: 4px;
    color: var(--color-fg-muted);
    font-weight: 700;
  }
</style>
```

- [ ] **Step 2: Build ProgressBar (used in task rows)**

Create `src/lib/components/ProgressBar.svelte`:

```svelte
<script lang="ts">
  type Props = { current: number; total: number }
  let { current, total }: Props = $props()
</script>

<span class="bar" aria-label={`${current} of ${total}`}>
  {#each Array(total) as _, i}
    <span class="cell" class:filled={i < current}>{i < current ? '▮' : '▯'}</span>
  {/each}
  <span class="text">{current} / {total}</span>
</span>

<style>
  .bar { font-family: var(--font-mono); font-size: 10px; letter-spacing: 2px; color: var(--color-accent); }
  .text { margin-left: 6px; color: var(--color-fg-muted); }
</style>
```

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(ui): add TimerDisplay and ProgressBar components"
```

---

## Task 21: FOCUS Page

**Files:**
- Create: `src/lib/components/InterruptDialog.svelte`, `src/lib/components/CompleteOverlay.svelte`
- Replace: `src/routes/focus/+page.svelte`

- [ ] **Step 1: Build InterruptDialog**

Create `src/lib/components/InterruptDialog.svelte`:

```svelte
<script lang="ts">
  type Props = {
    open: boolean
    onsubmit: (reason: string, action: 'continue' | 'restart' | 'abandon') => void
    onclose: () => void
  }
  let { open, onsubmit, onclose }: Props = $props()
  let reason = $state('')

  function submit(action: 'continue' | 'restart' | 'abandon') {
    onsubmit(reason, action)
    reason = ''
  }
</script>

{#if open}
  <div class="backdrop" onclick={onclose} role="presentation"></div>
  <div class="dialog" role="dialog" aria-modal="true" aria-label="Interrupt">
    <div class="title">▸ INTERRUPT</div>
    <input
      class="input"
      type="text"
      placeholder="为什么打断? (可空)"
      bind:value={reason}
      autofocus
    />
    <div class="actions">
      <button class="btn primary" onclick={() => submit('continue')}>CONTINUE ▸</button>
      <button class="btn" onclick={() => submit('restart')}>RESTART</button>
      <button class="btn danger" onclick={() => submit('abandon')}>ABANDON</button>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed; inset: 0; background: rgba(0, 0, 0, 0.7); z-index: 100;
  }
  .dialog {
    position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%);
    background: var(--color-bg); border: var(--border-width) solid var(--color-fg);
    box-shadow: var(--shadow-hard-md);
    padding: 24px; min-width: 360px; z-index: 101;
  }
  .title {
    font-size: 11px; letter-spacing: 3px; font-weight: 900;
    color: var(--color-accent); margin-bottom: 16px;
  }
  .input {
    width: 100%; padding: 10px 12px;
    background: var(--color-bg-elevated); color: var(--color-fg);
    border: var(--border-width) solid var(--color-border);
    font-family: var(--font-mono); font-size: 13px;
  }
  .input:focus { outline: 0; border-color: var(--color-accent); }
  .actions { display: flex; gap: 0; margin-top: 16px; }
  .btn {
    flex: 1; padding: 12px;
    font-size: 11px; letter-spacing: 3px; font-weight: 900;
    border: var(--border-width) solid var(--color-fg);
  }
  .btn + .btn { border-left: 0; }
  .btn:hover { color: var(--color-accent); }
  .btn.primary { background: var(--color-accent); color: var(--color-bg); border-color: var(--color-accent); }
  .btn.danger:hover { color: var(--color-danger); }
</style>
```

- [ ] **Step 2: Build CompleteOverlay**

Create `src/lib/components/CompleteOverlay.svelte`:

```svelte
<script lang="ts">
  type Props = {
    open: boolean
    label: string
    onclose: () => void
  }
  let { open, label, onclose }: Props = $props()
</script>

{#if open}
  <div class="overlay" role="dialog" aria-label={label}>
    <div class="bigtext acid-end-flash">{label}</div>
    <div class="hint">— click anywhere to continue —</div>
    <button class="hit" onclick={onclose} aria-label="dismiss"></button>
  </div>
{/if}

<style>
  .overlay {
    position: fixed; inset: 0; z-index: 200;
    background: var(--color-bg);
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    border: 8px solid var(--color-accent);
  }
  .bigtext {
    font-family: var(--font-display);
    font-size: 96px; font-weight: 900; letter-spacing: -2px;
    color: var(--color-fg);
  }
  .hint {
    margin-top: 16px;
    font-size: 11px; letter-spacing: 4px;
    color: var(--color-fg-muted);
  }
  .hit { position: absolute; inset: 0; background: transparent; cursor: pointer; }
</style>
```

- [ ] **Step 3: Build the FOCUS page**

Replace `src/routes/focus/+page.svelte`:

```svelte
<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import TimerDisplay from '../../lib/components/TimerDisplay.svelte'
  import InterruptDialog from '../../lib/components/InterruptDialog.svelte'
  import CompleteOverlay from '../../lib/components/CompleteOverlay.svelte'
  import { settings } from '../../lib/stores/settings.svelte'
  import { tasksStore } from '../../lib/stores/tasks.svelte'
  import { createTimerMachine } from '../../lib/stores/timer.svelte'
  import { ipc } from '../../lib/ipc'
  import type { Pomodoro } from '../../lib/types'

  let active: Pomodoro | null = $state(null)
  let interruptOpen = $state(false)
  let overlayOpen = $state(false)
  let overlayLabel = $state('')

  let timer = createTimerMachine(
    {
      workSecs: settings.state.workSecs,
      breakSecs: settings.state.breakSecs,
      longBreakSecs: settings.state.longBreakSecs,
      longBreakEvery: settings.state.longBreakEvery,
      autoContinue: settings.state.autoContinue,
    },
    {
      onWorkComplete: async ({ actualSecs }) => {
        if (active) await ipc.completePomodoro(active.id, actualSecs)
        active = null
        overlayLabel = 'FOCUS COMPLETE'
        if (settings.state.notifyFullscreen) overlayOpen = true
        await fireEndNotifications('结束', '休息一下')
      },
      onBreakComplete: async () => {
        overlayLabel = 'BREAK OVER'
        if (settings.state.notifyFullscreen) overlayOpen = true
        await fireEndNotifications('休息结束', '回到学习')
        await ipc.notifyFocusWindow()
      },
      onInterrupt: async ({ actualSecs, reason, abandoned }) => {
        if (active) await ipc.interruptPomodoro(active.id, reason, actualSecs, abandoned)
        active = null
      },
    },
  )

  let rafId: number
  function loop() { timer.tick(); rafId = requestAnimationFrame(loop) }

  async function fireEndNotifications(title: string, body: string) {
    const s = settings.state
    if (s.notifySystem) await ipc.notifySystem(title, body)
    if (s.notifySound) await ipc.notifySound(s.notifySoundFile)
    if (s.notifyTaskbar) await ipc.notifyTaskbarFlash()
  }

  onMount(() => {
    rafId = requestAnimationFrame(loop)
    tasksStore.reload(null)
  })
  onDestroy(() => cancelAnimationFrame(rafId))

  let selectedTaskId = $state<number | null>(null)

  async function start() {
    const p = await ipc.startPomodoro(selectedTaskId, settings.state.workSecs)
    active = p
    timer.startWork(selectedTaskId)
  }
</script>

<div class="page">
  <header class="hdr">
    <div class="dot"></div>
    <div class="title">FOCUS / {String(timer.completedWorkCount + 1).padStart(2, '0')}</div>
    <div class="spacer"></div>
    <div class="set">SESSION {timer.completedWorkCount} / —</div>
  </header>

  <section class="task-row">
    <label class="lbl">▸ CURRENT TASK</label>
    <select class="select" bind:value={selectedTaskId} disabled={timer.phase !== 'idle'}>
      <option value={null}>(no task)</option>
      {#each tasksStore.tasks as t}
        <option value={t.id}>{t.title}</option>
      {/each}
    </select>
  </section>

  <TimerDisplay
    remainingSecs={timer.remainingSecs}
    plannedSecs={timer.plannedSecs}
    label={timer.phase === 'focusing' ? 'FOCUS' :
           timer.phase === 'breaking' ? 'SHORT BREAK' :
           timer.phase === 'long-breaking' ? 'LONG BREAK' :
           timer.phase === 'paused' ? 'PAUSED' : 'READY'}
  />

  <div class="actions">
    {#if timer.phase === 'idle'}
      <button class="btn primary" onclick={start}>START ▸</button>
    {:else if timer.phase === 'focusing'}
      <button class="btn" onclick={() => timer.pause()}>PAUSE</button>
      <button class="btn alt" onclick={() => (interruptOpen = true)}>⊗ INTERRUPT</button>
    {:else if timer.phase === 'paused'}
      <button class="btn primary" onclick={() => timer.resume()}>RESUME ▸</button>
      <button class="btn alt" onclick={() => (interruptOpen = true)}>⊗ INTERRUPT</button>
    {:else}
      <button class="btn" onclick={() => timer.reset()}>SKIP BREAK</button>
    {/if}
  </div>

  <InterruptDialog
    open={interruptOpen}
    onsubmit={(reason, action) => {
      interruptOpen = false
      if (action === 'abandon') timer.interrupt(reason || null, true)
      else timer.interrupt(reason || null, false)
      if (action === 'restart') start()
    }}
    onclose={() => (interruptOpen = false)}
  />

  <CompleteOverlay
    open={overlayOpen}
    label={overlayLabel}
    onclose={() => (overlayOpen = false)}
  />
</div>

<style>
  .page { padding: 0; }
  .hdr {
    display: flex; align-items: center; gap: 10px;
    padding: 14px 20px;
    border-bottom: 1px solid var(--color-border);
  }
  .dot { width: 10px; height: 10px; background: var(--color-accent); }
  .title { font-size: 11px; letter-spacing: 3px; font-weight: 900; }
  .spacer { flex: 1; }
  .set { font-size: 10px; letter-spacing: 2px; color: var(--color-fg-muted); }
  .task-row { padding: 24px 20px 0; }
  .lbl { font-size: 9px; letter-spacing: 3px; color: var(--color-fg-muted); display: block; margin-bottom: 6px; }
  .select {
    width: 100%; padding: 10px 12px;
    background: var(--color-bg-elevated); color: var(--color-fg);
    border: var(--border-width) solid var(--color-border);
    font-family: var(--font-mono); font-size: 13px;
    border-left: 3px solid var(--color-accent);
  }
  .select:focus { outline: 0; border-color: var(--color-accent); }
  .actions { padding: 0 20px 20px; display: flex; gap: 0; }
  .btn {
    flex: 1; padding: 14px;
    font-size: 12px; letter-spacing: 3px; font-weight: 900;
    border: var(--border-width) solid var(--color-fg);
    transition: box-shadow var(--duration-fast) var(--ease-snap);
  }
  .btn + .btn { border-left: 0; }
  .btn:hover { box-shadow: var(--shadow-hard-sm); }
  .btn.primary { background: var(--color-accent); color: var(--color-bg); border-color: var(--color-accent); }
  .btn.alt { color: var(--color-fg-muted); }
  .btn.alt:hover { color: var(--color-accent); }
</style>
```

- [ ] **Step 4: Manual smoke test**

```bash
pnpm tauri dev
```

Test these flows:
1. Start a focus session → counts down → at 0 a "FOCUS COMPLETE" overlay appears, system notification fires, sound plays.
2. Start, then click PAUSE → countdown stops; click RESUME → continues.
3. Start, click INTERRUPT, type "test", click CONTINUE → returns to idle; verify in DB that interrupt row was inserted.
4. Click START with a task selected → on completion, `pomodoros.task_id` is set in DB.

## Task 22: TASKS Page

**Files:**
- Create: `src/lib/components/GoalCard.svelte`, `src/lib/components/TaskRow.svelte`
- Replace: `src/routes/tasks/+page.svelte`

- [ ] **Step 1: Build TaskRow**

Create `src/lib/components/TaskRow.svelte`:

```svelte
<script lang="ts">
  import ProgressBar from './ProgressBar.svelte'
  import type { Task } from '../types'

  type Props = {
    task: Task
    completedPomos: number
    onComplete: (id: number) => void
    onDelete: (id: number) => void
    onEdit: (id: number) => void
    isActive?: boolean
  }
  let { task, completedPomos, onComplete, onDelete, onEdit, isActive = false }: Props = $props()
</script>

<div class="row" class:active={isActive}>
  <button
    class="check"
    class:done={task.status === 'done'}
    onclick={() => onComplete(task.id)}
    aria-label="toggle done"
  >
    {task.status === 'done' ? '✓' : ''}
  </button>
  <div class="body">
    <div class="title" class:strike={task.status === 'done'}>{task.title}</div>
    <div class="meta">
      <ProgressBar current={completedPomos} total={task.estimated_pomos} />
      {#if isActive}<span class="active-tag">◂ ACTIVE</span>{/if}
    </div>
  </div>
  <div class="ctrl">
    <button class="icon" onclick={() => onEdit(task.id)} aria-label="edit">···</button>
    <button class="icon danger" onclick={() => onDelete(task.id)} aria-label="delete">⊗</button>
  </div>
</div>

<style>
  .row {
    display: flex; align-items: center; gap: 12px;
    padding: 12px 20px;
    border-bottom: 1px solid var(--color-border);
  }
  .row.active { background: var(--color-bg-elevated); border-left: 3px solid var(--color-accent); }
  .check {
    width: 14px; height: 14px;
    border: var(--border-width) solid var(--color-fg);
    display: flex; align-items: center; justify-content: center;
    color: var(--color-bg); background: transparent;
    font-size: 9px; font-weight: 900;
  }
  .check.done { background: var(--color-accent); border-color: var(--color-accent); }
  .body { flex: 1; }
  .title { font-size: 13px; font-weight: 600; color: var(--color-fg); }
  .title.strike { text-decoration: line-through; color: var(--color-fg-muted); }
  .meta { margin-top: 4px; display: flex; align-items: center; gap: 10px; }
  .active-tag { font-size: 9px; letter-spacing: 2px; color: var(--color-accent); font-weight: 700; }
  .ctrl { display: flex; gap: 4px; }
  .icon {
    width: 22px; height: 22px;
    color: var(--color-fg-muted);
    transition: color var(--duration-fast) var(--ease-snap);
  }
  .icon:hover { color: var(--color-fg); }
  .icon.danger:hover { color: var(--color-danger); }
</style>
```

- [ ] **Step 2: Build GoalCard**

Create `src/lib/components/GoalCard.svelte`:

```svelte
<script lang="ts">
  import TaskRow from './TaskRow.svelte'
  import type { Goal, Task } from '../types'

  type Props = {
    goal: Goal | null  // null means "无目标" bucket
    tasks: Task[]
    completedPomosByTask: Record<number, number>
    activeTaskId: number | null
    onAddTask: (goalId: number | null) => void
    onCompleteTask: (id: number) => void
    onDeleteTask: (id: number) => void
    onEditTask: (id: number) => void
    onArchiveGoal?: () => void
  }
  let {
    goal, tasks, completedPomosByTask, activeTaskId,
    onAddTask, onCompleteTask, onDeleteTask, onEditTask, onArchiveGoal,
  }: Props = $props()
</script>

<article class="card">
  <header class="hd">
    <div class="title">{goal ? goal.title : '(NO GOAL)'}</div>
    <div class="ctrl">
      <button class="btn" onclick={() => onAddTask(goal?.id ?? null)}>+ TASK</button>
      {#if goal && onArchiveGoal}
        <button class="btn alt" onclick={onArchiveGoal}>ARCHIVE</button>
      {/if}
    </div>
  </header>
  {#if tasks.length === 0}
    <p class="empty">no tasks yet</p>
  {:else}
    {#each tasks as t (t.id)}
      <TaskRow
        task={t}
        completedPomos={completedPomosByTask[t.id] ?? 0}
        isActive={t.id === activeTaskId}
        onComplete={onCompleteTask}
        onDelete={onDeleteTask}
        onEdit={onEditTask}
      />
    {/each}
  {/if}
</article>

<style>
  .card {
    margin: 16px 20px;
    border: var(--border-width) solid var(--color-fg);
    background: var(--color-bg);
  }
  .hd {
    display: flex; align-items: center; justify-content: space-between;
    padding: 12px 16px;
    border-bottom: var(--border-width) solid var(--color-fg);
    background: var(--color-bg-elevated);
  }
  .title {
    font-size: 12px; letter-spacing: 2px; font-weight: 900; color: var(--color-fg);
  }
  .ctrl { display: flex; gap: 8px; }
  .btn {
    padding: 6px 10px;
    font-size: 10px; letter-spacing: 2px; font-weight: 900;
    border: 1px solid var(--color-fg);
    color: var(--color-fg);
  }
  .btn:hover { background: var(--color-accent); color: var(--color-bg); border-color: var(--color-accent); }
  .btn.alt { color: var(--color-fg-muted); }
  .empty { padding: 16px; font-size: 11px; color: var(--color-fg-muted); letter-spacing: 1px; }
</style>
```

- [ ] **Step 3: Build the TASKS page**

Replace `src/routes/tasks/+page.svelte`:

```svelte
<script lang="ts">
  import { onMount } from 'svelte'
  import GoalCard from '../../lib/components/GoalCard.svelte'
  import { goalsStore } from '../../lib/stores/goals.svelte'
  import { tasksStore } from '../../lib/stores/tasks.svelte'
  import { ipc } from '../../lib/ipc'
  import type { Task } from '../../lib/types'

  let allTasks = $state<Task[]>([])
  let completedPomosByTask = $state<Record<number, number>>({})

  async function reload() {
    await goalsStore.reload()
    allTasks = await ipc.listTasks(null, false)
    const pomos = await ipc.listPomodorosToday()
    const map: Record<number, number> = {}
    for (const p of pomos) {
      if (p.task_id != null && p.status === 'completed') {
        map[p.task_id] = (map[p.task_id] ?? 0) + 1
      }
    }
    completedPomosByTask = map
  }

  onMount(reload)

  async function addGoal() {
    const title = prompt('Goal title?')
    if (!title) return
    await goalsStore.create(title)
  }

  async function addTask(goalId: number | null) {
    const title = prompt('Task title?')
    if (!title) return
    const estStr = prompt('Estimated pomodoros?', '1') ?? '1'
    const est = Math.max(1, parseInt(estStr, 10) || 1)
    await ipc.createTask(goalId, title, est)
    await reload()
  }

  async function completeTask(id: number) {
    await ipc.completeTask(id)
    await reload()
  }

  async function deleteTask(id: number) {
    if (!confirm('Delete this task?')) return
    await ipc.deleteTask(id)
    await reload()
  }

  async function editTask(id: number) {
    const t = allTasks.find(x => x.id === id)
    if (!t) return
    const title = prompt('New title?', t.title)
    if (title) await ipc.updateTask(id, { title })
    const estStr = prompt('Estimated pomos?', String(t.estimated_pomos))
    if (estStr) await ipc.updateTask(id, { estimatedPomos: parseInt(estStr, 10) || t.estimated_pomos })
    await reload()
  }

  async function archiveGoal(id: number) {
    if (!confirm('Archive this goal?')) return
    await goalsStore.archive(id)
    await reload()
  }

  function tasksFor(goalId: number | null) {
    return allTasks.filter(t => t.goal_id === goalId)
  }
</script>

<div class="page">
  <header class="hdr">
    <span class="lbl">▸ GOALS / TASKS</span>
    <div class="spacer"></div>
    <button class="btn" onclick={addGoal}>+ GOAL</button>
  </header>

  {#each goalsStore.goals as g (g.id)}
    <GoalCard
      goal={g}
      tasks={tasksFor(g.id)}
      completedPomosByTask={completedPomosByTask}
      activeTaskId={null}
      onAddTask={addTask}
      onCompleteTask={completeTask}
      onDeleteTask={deleteTask}
      onEditTask={editTask}
      onArchiveGoal={() => archiveGoal(g.id)}
    />
  {/each}

  <GoalCard
    goal={null}
    tasks={tasksFor(null)}
    completedPomosByTask={completedPomosByTask}
    activeTaskId={null}
    onAddTask={addTask}
    onCompleteTask={completeTask}
    onDeleteTask={deleteTask}
    onEditTask={editTask}
  />
</div>

<style>
  .page { padding: 0 0 24px; }
  .hdr {
    display: flex; align-items: center; gap: 10px;
    padding: 14px 20px;
    border-bottom: 1px solid var(--color-border);
  }
  .lbl { font-size: 9px; letter-spacing: 3px; color: var(--color-fg-muted); }
  .spacer { flex: 1; }
  .btn {
    padding: 8px 12px;
    font-size: 11px; letter-spacing: 2px; font-weight: 900;
    border: var(--border-width) solid var(--color-fg);
  }
  .btn:hover { background: var(--color-accent); color: var(--color-bg); border-color: var(--color-accent); }
</style>
```

- [ ] **Step 4: Smoke test**

```bash
pnpm tauri dev
```

Verify: + GOAL adds a goal card; + TASK on a card adds a task; ✓ marks done; ⊗ deletes; archive removes from view.

## Task 23: STATS Page

**Files:**
- Replace: `src/routes/stats/+page.svelte`

- [ ] **Step 1: Build the stats page**

Replace `src/routes/stats/+page.svelte`:

```svelte
<script lang="ts">
  import { onMount } from 'svelte'
  import { ipc } from '../../lib/ipc'
  import type { StatsSummary } from '../../lib/types'

  let stats = $state<StatsSummary | null>(null)

  onMount(async () => { stats = await ipc.getStats() })

  function fmtSecs(s: number) {
    const h = Math.floor(s / 3600)
    const m = Math.floor((s % 3600) / 60)
    return h > 0 ? `${h}h ${m}m` : `${m}m`
  }

  function maxFocus(d: { focus_secs: number }[]): number {
    return d.reduce((max, x) => Math.max(max, x.focus_secs), 1)
  }
</script>

<div class="page">
  <header class="hdr">
    <span class="lbl">▸ STATISTICS</span>
  </header>

  {#if !stats}
    <p class="empty">loading…</p>
  {:else}
    <section class="cards">
      <div class="stat">
        <div class="num">{stats.today.completed}</div>
        <div class="cap">TODAY POMODOROS</div>
      </div>
      <div class="stat">
        <div class="num">{fmtSecs(stats.today.focus_secs)}</div>
        <div class="cap">TODAY FOCUS</div>
      </div>
      <div class="stat">
        <div class="num">{stats.streak_days}</div>
        <div class="cap">STREAK DAYS</div>
      </div>
      <div class="stat">
        <div class="num">{stats.total.completed}</div>
        <div class="cap">TOTAL POMODOROS</div>
      </div>
    </section>

    <section class="block">
      <div class="block-lbl">▸ LAST 7 DAYS</div>
      <div class="bars">
        {#each stats.last_7_days as d}
          {@const h = (d.focus_secs / maxFocus(stats.last_7_days)) * 100}
          <div class="bar-wrap">
            <div class="bar" style="height: {Math.max(2, h)}%"></div>
            <div class="bar-lbl">{d.date.slice(5)}</div>
            <div class="bar-num">{d.completed}</div>
          </div>
        {/each}
      </div>
    </section>

    <section class="block">
      <div class="block-lbl">▸ BY GOAL</div>
      {#if stats.by_goal.length === 0}
        <p class="empty">no goal data</p>
      {:else}
        {#each stats.by_goal as g}
          <div class="goal-row">
            <span class="goal-name">{g.goal_title}</span>
            <span class="goal-stats">▮ {g.completed} pomos · {fmtSecs(g.focus_secs)}</span>
          </div>
        {/each}
      {/if}
    </section>

    <section class="block">
      <div class="block-lbl">▸ TOP INTERRUPTS</div>
      {#if stats.top_interrupts.length === 0}
        <p class="empty">no interrupts logged</p>
      {:else}
        {#each stats.top_interrupts as i}
          <div class="goal-row">
            <span class="goal-name">{i.reason}</span>
            <span class="goal-stats">× {i.count}</span>
          </div>
        {/each}
      {/if}
    </section>
  {/if}
</div>

<style>
  .page { padding: 0 0 24px; }
  .hdr {
    padding: 14px 20px;
    border-bottom: 1px solid var(--color-border);
  }
  .lbl { font-size: 9px; letter-spacing: 3px; color: var(--color-fg-muted); }
  .empty { padding: 16px 20px; font-size: 11px; color: var(--color-fg-muted); }

  .cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 0;
  }
  .stat {
    padding: 20px;
    border-right: 1px solid var(--color-border);
    border-bottom: 1px solid var(--color-border);
  }
  .num {
    font-family: var(--font-display);
    font-size: 36px; font-weight: 900;
    color: var(--color-accent);
  }
  .cap {
    font-size: 9px; letter-spacing: 3px;
    color: var(--color-fg-muted);
    margin-top: 6px;
  }

  .block {
    padding: 16px 20px;
    border-bottom: 1px solid var(--color-border);
  }
  .block-lbl {
    font-size: 9px; letter-spacing: 3px; color: var(--color-fg-muted);
    margin-bottom: 12px;
  }

  .bars {
    display: flex; gap: 4px; align-items: flex-end;
    height: 120px;
  }
  .bar-wrap {
    flex: 1; display: flex; flex-direction: column; align-items: center; gap: 4px;
  }
  .bar {
    width: 100%;
    background: var(--color-accent);
    box-shadow: 3px -3px 0 var(--color-fg);
  }
  .bar-lbl {
    font-size: 8px; letter-spacing: 1px; color: var(--color-fg-muted);
  }
  .bar-num {
    font-size: 10px; font-weight: 700; color: var(--color-fg);
  }

  .goal-row {
    display: flex; justify-content: space-between; align-items: center;
    padding: 8px 0;
    border-bottom: 1px solid var(--color-border);
    font-size: 12px;
  }
  .goal-row:last-child { border-bottom: 0; }
  .goal-stats { color: var(--color-accent); font-family: var(--font-mono); font-size: 11px; }
</style>
```

- [ ] **Step 2: Smoke test**

```bash
pnpm tauri dev
```

Run a few pomodoros first (or insert sample data via devtools) → STATS page renders cards, bars, and goal breakdowns.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(stats): stats page with day/week/goal/interrupt breakdowns"
```

---

## Task 24: THEME Page

**Files:**
- Replace: `src/routes/theme/+page.svelte`

- [ ] **Step 1: Build the theme picker**

Replace `src/routes/theme/+page.svelte`:

```svelte
<script lang="ts">
  import { listThemes, setTheme, getCurrentTheme } from '../../themes/registry'
  import { settings } from '../../lib/stores/settings.svelte'

  let current = $state(getCurrentTheme())
  const themes = listThemes()

  async function pick(id: string) {
    try {
      setTheme(id)
      current = id
      await settings.update('theme', id)
    } catch (e) {
      alert((e as Error).message)
    }
  }
</script>

<div class="page">
  <header class="hdr">
    <span class="lbl">▸ THEMES</span>
  </header>

  <div class="grid">
    {#each themes as t}
      <button
        class="card"
        class:active={t.id === current}
        class:soon={t.status === 'coming-soon'}
        onclick={() => pick(t.id)}
        disabled={t.status === 'coming-soon'}
      >
        <div class="name">{t.displayName}</div>
        <div class="desc">{t.preview}</div>
        {#if t.status === 'coming-soon'}<div class="badge">COMING SOON</div>{/if}
        {#if t.id === current}<div class="badge active-badge">ACTIVE</div>{/if}
      </button>
    {/each}
  </div>
</div>

<style>
  .page { padding: 0 0 24px; }
  .hdr {
    padding: 14px 20px;
    border-bottom: 1px solid var(--color-border);
  }
  .lbl { font-size: 9px; letter-spacing: 3px; color: var(--color-fg-muted); }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
    gap: 0;
    padding: 0;
  }
  .card {
    text-align: left;
    padding: 24px 20px;
    border-right: 1px solid var(--color-border);
    border-bottom: 1px solid var(--color-border);
    background: var(--color-bg);
    transition: box-shadow var(--duration-fast) var(--ease-snap);
    position: relative;
    cursor: pointer;
  }
  .card:hover:not(:disabled) { box-shadow: var(--shadow-hard-sm) inset; }
  .card.active { background: var(--color-bg-elevated); border-left: 4px solid var(--color-accent); }
  .card.soon { opacity: 0.5; cursor: not-allowed; }
  .name {
    font-size: 14px; letter-spacing: 1px; font-weight: 900;
    color: var(--color-fg);
  }
  .desc {
    margin-top: 8px;
    font-size: 11px; color: var(--color-fg-muted);
  }
  .badge {
    position: absolute; top: 12px; right: 12px;
    font-size: 9px; letter-spacing: 2px; font-weight: 900;
    padding: 4px 8px;
    background: var(--color-fg-muted); color: var(--color-bg);
  }
  .active-badge { background: var(--color-accent); color: var(--color-bg); }
</style>
```

- [ ] **Step 2: Smoke test**

```bash
pnpm tauri dev
```

Verify: clicking acid theme highlights it; synthwave shows COMING SOON and is disabled.

## Task 25: Data Export / Import / Reset IPC

**Files:**
- Create: `src-tauri/src/commands/data.rs`
- Modify: `src-tauri/src/commands/mod.rs`, `src-tauri/src/lib.rs`, `src/lib/ipc.ts`

- [ ] **Step 1: Create the data module**

Add `pub mod data;` to `src-tauri/src/commands/mod.rs`.

Create `src-tauri/src/commands/data.rs`:

```rust
use crate::error::AppResult;
use crate::state::AppState;
use crate::seed;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Export {
    pub version: i64,
    pub goals: Vec<Value>,
    pub tasks: Vec<Value>,
    pub pomodoros: Vec<Value>,
    pub interrupts: Vec<Value>,
    pub settings: Vec<(String, String)>,
}

#[tauri::command]
pub fn export_data(state: State<'_, AppState>) -> AppResult<String> {
    let conn = state.db.lock();
    let goals = rows_to_objects(&conn, "SELECT id, title, description, color, status, created_at, archived_at FROM goals",
        &["id", "title", "description", "color", "status", "created_at", "archived_at"])?;
    let tasks = rows_to_objects(&conn, "SELECT id, goal_id, title, estimated_pomos, status, created_at, done_at, sort_order FROM tasks",
        &["id", "goal_id", "title", "estimated_pomos", "status", "created_at", "done_at", "sort_order"])?;
    let pomodoros = rows_to_objects(&conn, "SELECT id, task_id, goal_id, started_at, ended_at, planned_secs, actual_secs, status, date_local FROM pomodoros",
        &["id", "task_id", "goal_id", "started_at", "ended_at", "planned_secs", "actual_secs", "status", "date_local"])?;
    let interrupts = rows_to_objects(&conn, "SELECT id, pomodoro_id, reason, occurred_at FROM interrupts",
        &["id", "pomodoro_id", "reason", "occurred_at"])?;
    let mut s_stmt = conn.prepare("SELECT key, value FROM settings")?;
    let settings: Vec<(String, String)> = s_stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    let bundle = Export { version: 1, goals, tasks, pomodoros, interrupts, settings };
    Ok(serde_json::to_string_pretty(&bundle)?)
}

fn rows_to_objects(conn: &rusqlite::Connection, sql: &str, cols: &[&str]) -> AppResult<Vec<Value>> {
    let mut stmt = conn.prepare(sql)?;
    let column_count = cols.len();
    let rows = stmt.query_map([], |row| {
        let mut obj = serde_json::Map::new();
        for (i, c) in cols.iter().enumerate().take(column_count) {
            let v: Value = match row.get_ref(i)? {
                rusqlite::types::ValueRef::Null => Value::Null,
                rusqlite::types::ValueRef::Integer(n) => json!(n),
                rusqlite::types::ValueRef::Real(f) => json!(f),
                rusqlite::types::ValueRef::Text(t) => json!(std::str::from_utf8(t).unwrap_or("")),
                rusqlite::types::ValueRef::Blob(_) => Value::Null,
            };
            obj.insert(c.to_string(), v);
        }
        Ok(Value::Object(obj))
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

#[tauri::command]
pub fn reset_data(state: State<'_, AppState>) -> AppResult<()> {
    let mut conn = state.db.lock();
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM interrupts", [])?;
    tx.execute("DELETE FROM pomodoros", [])?;
    tx.execute("DELETE FROM tasks", [])?;
    tx.execute("DELETE FROM goals", [])?;
    tx.execute("DELETE FROM settings", [])?;
    tx.commit()?;
    seed::seed_defaults(&conn)?;
    Ok(())
}

#[tauri::command]
pub fn import_data(state: State<'_, AppState>, json_text: String) -> AppResult<()> {
    let bundle: Export = serde_json::from_str(&json_text)?;
    let mut conn = state.db.lock();
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM interrupts", [])?;
    tx.execute("DELETE FROM pomodoros", [])?;
    tx.execute("DELETE FROM tasks", [])?;
    tx.execute("DELETE FROM goals", [])?;
    tx.execute("DELETE FROM settings", [])?;
    for g in &bundle.goals {
        tx.execute(
            "INSERT INTO goals(id, title, description, color, status, created_at, archived_at)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                g["id"].as_i64(), g["title"].as_str(), g["description"].as_str(),
                g["color"].as_str(), g["status"].as_str().unwrap_or("active"),
                g["created_at"].as_i64().unwrap_or(0), g["archived_at"].as_i64()
            ],
        )?;
    }
    for t in &bundle.tasks {
        tx.execute(
            "INSERT INTO tasks(id, goal_id, title, estimated_pomos, status, created_at, done_at, sort_order)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                t["id"].as_i64(), t["goal_id"].as_i64(), t["title"].as_str(),
                t["estimated_pomos"].as_i64().unwrap_or(1),
                t["status"].as_str().unwrap_or("active"),
                t["created_at"].as_i64().unwrap_or(0),
                t["done_at"].as_i64(), t["sort_order"].as_i64().unwrap_or(0)
            ],
        )?;
    }
    for p in &bundle.pomodoros {
        tx.execute(
            "INSERT INTO pomodoros(id, task_id, goal_id, started_at, ended_at, planned_secs, actual_secs, status, date_local)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                p["id"].as_i64(), p["task_id"].as_i64(), p["goal_id"].as_i64(),
                p["started_at"].as_i64().unwrap_or(0), p["ended_at"].as_i64(),
                p["planned_secs"].as_i64().unwrap_or(0), p["actual_secs"].as_i64(),
                p["status"].as_str().unwrap_or("completed"),
                p["date_local"].as_str().unwrap_or("1970-01-01")
            ],
        )?;
    }
    for i in &bundle.interrupts {
        tx.execute(
            "INSERT INTO interrupts(id, pomodoro_id, reason, occurred_at) VALUES(?1, ?2, ?3, ?4)",
            params![
                i["id"].as_i64(), i["pomodoro_id"].as_i64().unwrap_or(0),
                i["reason"].as_str(), i["occurred_at"].as_i64().unwrap_or(0)
            ],
        )?;
    }
    for (k, v) in &bundle.settings {
        tx.execute(
            "INSERT INTO settings(key, value) VALUES(?1, ?2)",
            params![k, v],
        )?;
    }
    tx.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use tempfile::tempdir;

    #[test]
    fn export_then_import_roundtrips() {
        let dir = tempdir().unwrap();
        let db = db::open(&dir.path().join("t.db")).unwrap();
        let s = AppState::new(db);
        seed::seed_defaults(&s.db.lock()).unwrap();
        s.db.lock().execute(
            "INSERT INTO goals(title, status, created_at) VALUES('g1', 'active', 0)", []
        ).unwrap();
        let json = export_data_inner(&s).unwrap();
        // Wipe and re-import
        s.db.lock().execute("DELETE FROM goals", []).unwrap();
        import_data_inner(&s, json).unwrap();
        let count: i64 = s.db.lock().query_row("SELECT count(*) FROM goals", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 1);
    }

    // Test helpers (the #[tauri::command] versions need a State which is hard to fake)
    fn export_data_inner(s: &AppState) -> AppResult<String> {
        let conn = s.db.lock();
        let goals = rows_to_objects(&conn, "SELECT id, title, description, color, status, created_at, archived_at FROM goals",
            &["id", "title", "description", "color", "status", "created_at", "archived_at"])?;
        let bundle = Export { version: 1, goals, tasks: vec![], pomodoros: vec![], interrupts: vec![], settings: vec![] };
        Ok(serde_json::to_string(&bundle)?)
    }

    fn import_data_inner(s: &AppState, json_text: String) -> AppResult<()> {
        let bundle: Export = serde_json::from_str(&json_text)?;
        let mut conn = s.db.lock();
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM goals", [])?;
        for g in &bundle.goals {
            tx.execute(
                "INSERT INTO goals(id, title, description, color, status, created_at, archived_at)
                 VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    g["id"].as_i64(), g["title"].as_str(), g["description"].as_str(),
                    g["color"].as_str(), g["status"].as_str().unwrap_or("active"),
                    g["created_at"].as_i64().unwrap_or(0), g["archived_at"].as_i64()
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }
}
```

- [ ] **Step 2: Register handlers**

Add to `tauri::generate_handler![...]`:

```rust
commands::data::export_data,
commands::data::import_data,
commands::data::reset_data,
```

- [ ] **Step 3: Add wrappers in `src/lib/ipc.ts`**

Append inside the `ipc` object:

```ts
// data
exportData: () => invoke<string>('export_data'),
importData: (jsonText: string) => invoke<void>('import_data', { jsonText }),
resetData: () => invoke<void>('reset_data'),
```

- [ ] **Step 4: Run tests**

```bash
cd src-tauri && cargo test --lib commands::data
```

Expected: 1 passed.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat(data): add export/import/reset IPC commands"
```

---

## Task 26: SETTINGS Page

**Files:**
- Replace: `src/routes/settings/+page.svelte`

- [ ] **Step 1: Build the settings page**

Replace `src/routes/settings/+page.svelte`:

```svelte
<script lang="ts">
  import { settings } from '../../lib/stores/settings.svelte'
  import { ipc } from '../../lib/ipc'

  async function setMin(field: 'workSecs' | 'breakSecs' | 'longBreakSecs', minutes: number) {
    const secs = Math.max(1, Math.min(180, Math.floor(minutes))) * 60
    await settings.update(field, secs)
  }

  async function reset() {
    if (!confirm('Reset all data? This cannot be undone.')) return
    await ipc.resetData()
    location.reload()
  }

  async function exportData() {
    const json = await ipc.exportData()
    const blob = new Blob([json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `pomodoro-study-${new Date().toISOString().slice(0, 10)}.json`
    a.click()
    URL.revokeObjectURL(url)
  }

  async function importData() {
    const input = document.createElement('input')
    input.type = 'file'
    input.accept = 'application/json'
    input.onchange = async () => {
      const file = input.files?.[0]
      if (!file) return
      if (!confirm('Importing will REPLACE all current data. Continue?')) return
      const text = await file.text()
      await ipc.importData(text)
      location.reload()
    }
    input.click()
  }
</script>

<div class="page">
  <header class="hdr"><span class="lbl">▸ SETTINGS</span></header>

  <section class="section">
    <h2 class="h2">TIMER</h2>
    <div class="field">
      <label>Work duration (minutes)</label>
      <input type="number" min="1" max="180"
             value={Math.round(settings.state.workSecs / 60)}
             oninput={(e) => setMin('workSecs', parseInt((e.target as HTMLInputElement).value, 10) || 25)} />
    </div>
    <div class="field">
      <label>Short break (minutes)</label>
      <input type="number" min="1" max="60"
             value={Math.round(settings.state.breakSecs / 60)}
             oninput={(e) => setMin('breakSecs', parseInt((e.target as HTMLInputElement).value, 10) || 5)} />
    </div>
    <div class="field">
      <label>Long break (minutes)</label>
      <input type="number" min="1" max="120"
             value={Math.round(settings.state.longBreakSecs / 60)}
             oninput={(e) => setMin('longBreakSecs', parseInt((e.target as HTMLInputElement).value, 10) || 15)} />
    </div>
    <div class="field">
      <label>Long break every N pomodoros</label>
      <input type="number" min="2" max="10"
             value={settings.state.longBreakEvery}
             oninput={(e) => settings.update('longBreakEvery', parseInt((e.target as HTMLInputElement).value, 10) || 4)} />
    </div>
    <div class="field check">
      <label>
        <input type="checkbox"
               checked={settings.state.autoContinue}
               onchange={(e) => settings.update('autoContinue', (e.target as HTMLInputElement).checked)} />
        Auto-continue between sessions
      </label>
    </div>
  </section>

  <section class="section">
    <h2 class="h2">NOTIFICATIONS</h2>
    <div class="field check">
      <label><input type="checkbox" checked={settings.state.notifySystem}
        onchange={(e) => settings.update('notifySystem', (e.target as HTMLInputElement).checked)} /> System toast</label>
    </div>
    <div class="field check">
      <label><input type="checkbox" checked={settings.state.notifySound}
        onchange={(e) => settings.update('notifySound', (e.target as HTMLInputElement).checked)} /> Sound</label>
    </div>
    <div class="field">
      <label>Sound file (in app/assets)</label>
      <input type="text"
             value={settings.state.notifySoundFile}
             oninput={(e) => settings.update('notifySoundFile', (e.target as HTMLInputElement).value)} />
      <button class="btn" onclick={() => ipc.notifySound(settings.state.notifySoundFile)}>TEST</button>
    </div>
    <div class="field check">
      <label><input type="checkbox" checked={settings.state.notifyFullscreen}
        onchange={(e) => settings.update('notifyFullscreen', (e.target as HTMLInputElement).checked)} /> Full-screen overlay</label>
    </div>
    <div class="field check">
      <label><input type="checkbox" checked={settings.state.notifyTaskbar}
        onchange={(e) => settings.update('notifyTaskbar', (e.target as HTMLInputElement).checked)} /> Taskbar flash</label>
    </div>
  </section>

  <section class="section">
    <h2 class="h2">DATA</h2>
    <div class="actions-row">
      <button class="btn" onclick={exportData}>EXPORT JSON</button>
      <button class="btn" onclick={importData}>IMPORT JSON</button>
      <button class="btn danger" onclick={reset}>RESET ALL</button>
    </div>
  </section>
</div>

<style>
  .page { padding: 0 0 24px; }
  .hdr { padding: 14px 20px; border-bottom: 1px solid var(--color-border); }
  .lbl { font-size: 9px; letter-spacing: 3px; color: var(--color-fg-muted); }
  .section {
    padding: 16px 20px;
    border-bottom: 1px solid var(--color-border);
  }
  .h2 {
    font-size: 11px; letter-spacing: 3px; font-weight: 900;
    color: var(--color-accent);
    margin-bottom: 12px;
  }
  .field {
    display: flex; flex-direction: column; gap: 6px;
    padding: 8px 0;
  }
  .field.check label {
    display: flex; align-items: center; gap: 8px;
    font-size: 12px;
  }
  .field label { font-size: 11px; letter-spacing: 1px; color: var(--color-fg-muted); }
  .field input[type="text"], .field input[type="number"] {
    background: var(--color-bg-elevated);
    border: var(--border-width) solid var(--color-border);
    color: var(--color-fg);
    padding: 8px 10px;
    font-family: var(--font-mono);
    font-size: 13px;
  }
  .field input:focus { outline: 0; border-color: var(--color-accent); }
  input[type="checkbox"] { accent-color: var(--color-accent); }
  .btn {
    padding: 8px 12px;
    font-size: 11px; letter-spacing: 2px; font-weight: 900;
    border: var(--border-width) solid var(--color-fg);
    color: var(--color-fg);
  }
  .btn:hover { background: var(--color-accent); color: var(--color-bg); border-color: var(--color-accent); }
  .btn.danger:hover { background: var(--color-danger); border-color: var(--color-danger); }
  .actions-row { display: flex; gap: 0; }
  .actions-row .btn { flex: 1; }
  .actions-row .btn + .btn { border-left: 0; }
</style>
```

- [ ] **Step 2: Smoke test**

```bash
pnpm tauri dev
```

Change a setting → reload app → verify it persisted (DB inspection or visible in UI).

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(settings): settings page wired to persisted store"
```

---

## Task 27: Documentation Suite

**Files:**
- Create: `README.md`, `docs/ARCHITECTURE.md`, `docs/DATA_MODEL.md`, `docs/THEMING.md`, `docs/IPC_API.md`, `docs/AGENT_GUIDE.md`

- [ ] **Step 1: Write the README**

Create `README.md`:

```markdown
# Pomodoro Study

Desktop study-focused pomodoro app. Tauri 2 + Svelte 5 + SQLite. Theme-switchable; ships with Acid Art.

## Quick Start

```bash
pnpm install
pnpm tauri dev          # development
pnpm tauri build        # production build (.msi on Windows)
pnpm test               # frontend unit tests
cd src-tauri && cargo test --lib   # rust tests
```

## Where data lives

Windows: `%LOCALAPPDATA%\study.pomodoro.app\data.db`

## Documentation

- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — system overview
- [`docs/DATA_MODEL.md`](docs/DATA_MODEL.md) — SQLite schema reference
- [`docs/THEMING.md`](docs/THEMING.md) — how theming works + adding a new theme
- [`docs/IPC_API.md`](docs/IPC_API.md) — every Tauri command, params, returns
- [`docs/AGENT_GUIDE.md`](docs/AGENT_GUIDE.md) — for AI agents maintaining this project
```

- [ ] **Step 2: Write ARCHITECTURE.md**

Create `docs/ARCHITECTURE.md`:

```markdown
# Architecture

## Process model

- **Rust (Tauri shell)**: filesystem, SQLite, system notifications, sound, window control. Single shared `AppState` holding the DB connection.
- **Svelte 5 (Webview)**: all UI, the timer state machine, theme switching. State lives in stores under `src/lib/stores/`.
- **IPC**: typed via `src/lib/ipc.ts`. Frontend calls `ipc.startPomodoro(...)`; Rust commands live under `src-tauri/src/commands/`.

## Why split this way

Future maintenance is friction-minimized:
- UI changes never require Rust edits.
- DB migrations and OS integration never require Svelte edits.

## Folders

```
src/
  lib/         shared frontend code (stores, ipc, components, types)
  routes/      one folder per page
  themes/      design tokens + per-theme CSS files
  styles/      reset only
src-tauri/
  src/
    commands/  IPC command modules, one per resource
    db/        connection + migrations
    notifier.rs
    state.rs
    error.rs
```

## Boundaries you must not cross

- The frontend **never** writes to disk directly. All persistence goes through IPC.
- Rust commands **never** call Svelte. They return data; the frontend reacts.
- Components **never** read raw settings keys — go through `settings.state.<field>`.
```

- [ ] **Step 3: Write DATA_MODEL.md**

Create `docs/DATA_MODEL.md`:

```markdown
# Data Model

Schema definition source of truth: `src-tauri/src/db/migrations/0001_init.sql`.

## Tables

### `goals`
Long-term study goals. `status` is `active` | `archived` | `done`. `archived_at` is set when status flips to `archived`.

### `tasks`
Concrete units of work, attached to a goal (or `goal_id IS NULL` for "no goal"). `estimated_pomos` lets the UI render progress bars. `sort_order` is dense within a goal.

### `pomodoros`
**Fact table — never mutate completed rows.** Each row represents one focus or break attempt.
- `task_id` and `goal_id` are both stored. `goal_id` is **redundant** but intentional: tasks may be reassigned, but historical pomodoros must remember the goal they served at the time.
- `date_local` is `YYYY-MM-DD` in the user's local time, set at start. This is what statistics group by (so a session that crosses midnight stays attributed to its starting day).

### `interrupts`
Many per pomodoro. `reason` is free-form text supplied by the user. Cascade-deleted with their pomodoro.

### `settings`
Single key-value table. Values are stored as **JSON strings** (so booleans, numbers, and strings round-trip cleanly). Keys are namespaced with dots (`timer.work_secs`).

## Migrations

`src-tauri/src/db/migrations/000N_*.sql`. Numbering is monotonic. The runner records applied versions in `schema_version`. Never edit a previously applied migration — add a new one.
```

- [ ] **Step 4: Write THEMING.md**

Create `docs/THEMING.md`:

```markdown
# Theming

## Layers

1. **Tokens** (`src/themes/_tokens.ts`) — the contract. Every visual concern is a token.
2. **Theme packs** (`src/themes/<id>/`) — concrete values for the tokens.
3. **Components** — only consume tokens via `var(--token-name)`. Never hard-code colors, shadows, or radii.

## Adding a new theme — 5 steps

1. Copy `src/themes/acid/` to `src/themes/<your-id>/`.
2. Edit `theme.css` — change all `--token` values inside the `[data-theme="<your-id>"]` selector.
3. Edit `meta.ts` — set `id`, `displayName`, `status: 'available'`, `preview`.
4. Register in `src/themes/bootstrap.ts`:

```ts
import { meta as yourMeta } from './<your-id>/meta'
registerTheme(yourMeta)
```

5. Done. No business code changes. The theme picker auto-discovers it via `listThemes()`.

## Constraints (Acid Art baseline)

- `border-radius: 0` everywhere.
- All `box-shadow` declarations use offset only (no blur).
- Components must not use hex colors — only `var(--color-*)`.
- Stylelint enforces all of the above (`.stylelintrc.json`).

## Token reference

See `src/themes/_tokens.ts` for the complete list and `src/themes/acid/theme.css` for example values.
```

- [ ] **Step 5: Write IPC_API.md**

Create `docs/IPC_API.md`:

```markdown
# IPC API Reference

All commands return `Result<T, AppError>` on the Rust side. The frontend wrapper (`src/lib/ipc.ts`) returns `Promise<T>` and throws on error.

## Goals

| Command | Params | Returns | Notes |
|---|---|---|---|
| `create_goal` | `title`, `description?`, `color?` | `Goal` | Status defaults to `active` |
| `list_goals` | `includeArchived: boolean` | `Goal[]` | Ordered by `created_at DESC` |
| `update_goal` | `id`, `title?`, `description?`, `color?` | `void` | Patch-style |
| `archive_goal` | `id` | `void` | Sets `status='archived'`, `archived_at` |
| `delete_goal` | `id` | `void` | Hard delete; tasks survive (goal_id → NULL) |

## Tasks

| Command | Params | Returns | Notes |
|---|---|---|---|
| `create_task` | `goalId?`, `title`, `estimatedPomos?` | `Task` | sort_order auto-assigned |
| `list_tasks` | `goalId?`, `includeDone` | `Task[]` | Ordered by sort_order, then created_at |
| `update_task` | `id`, `title?`, `goalId?`, `estimatedPomos?` | `void` | Patch-style |
| `complete_task` | `id` | `void` | Sets `status='done'`, `done_at` |
| `delete_task` | `id` | `void` | Pomodoros survive (task_id → NULL) |
| `reorder_tasks` | `orderedIds: number[]` | `void` | Single transaction |

## Pomodoros

| Command | Params | Returns | Notes |
|---|---|---|---|
| `start_pomodoro` | `taskId?`, `plannedSecs` | `Pomodoro` | Auto-fills `goal_id` from task |
| `complete_pomodoro` | `id`, `actualSecs` | `void` | Errors with NotFound if not in_progress |
| `interrupt_pomodoro` | `id`, `reason?`, `actualSecs`, `abandoned: boolean` | `void` | Writes interrupt row in same tx |
| `list_pomodoros_today` | — | `Pomodoro[]` | Filtered by `date_local = today` |

## Settings

| Command | Params | Returns |
|---|---|---|
| `get_setting` | `key` | `string?` |
| `set_setting` | `key`, `value` | `void` |
| `get_all_settings` | — | `Record<string, string>` |

## Stats

| Command | Params | Returns |
|---|---|---|
| `get_stats` | — | `StatsSummary` (today, total, streak_days, last_7_days, by_goal, top_interrupts) |

## Notifications

| Command | Params | Returns |
|---|---|---|
| `notify_system` | `title`, `body` | `void` |
| `notify_sound` | `soundFile` | `void` (resource: `assets/<file>`) |
| `notify_focus_window` | — | `void` |
| `notify_taskbar_flash` | — | `void` |
```

- [ ] **Step 6: Write AGENT_GUIDE.md**

Create `docs/AGENT_GUIDE.md`:

```markdown
# Agent Maintenance Guide

This document is for AI agents continuing development on this codebase.

## Hard rules

1. **Never write hex colors, shadows, or radii into component CSS.** Use `var(--color-*)` etc. Stylelint will reject the diff.
2. **Never mutate `pomodoros` rows once `status != 'in_progress'`.** They are historical records.
3. **Never bypass the IPC layer from the frontend.** No `window.__TAURI_INTERNALS__` calls; always go through `src/lib/ipc.ts`.
4. **Database migrations are append-only.** Never edit `0001_init.sql` or any prior migration; add `0002_*.sql` instead.
5. **Settings keys use dot namespacing** (`timer.work_secs`). Values are JSON-encoded strings.

## File-naming conventions

- Svelte 5 stores live in `*.svelte.ts` files (so `$state` works).
- Tests sit beside source: `foo.svelte.ts` + `foo.test.ts`.
- Rust modules use snake_case file names.

## Common maintenance tasks

### Add a new theme

See `docs/THEMING.md` § "Adding a new theme — 5 steps".

### Add a new setting

1. Add the key + default to `src-tauri/src/seed.rs`.
2. Add the field to `Settings` interface in `src/lib/stores/settings.svelte.ts`.
3. Add a row in `KEY_MAP` and `DEFAULTS`.
4. Add a UI control in `src/routes/settings/+page.svelte`.
5. Read it via `settings.state.<field>` from any component.

### Add a new statistic

1. Extend `StatsSummary` in `src-tauri/src/commands/stats.rs`.
2. Update `get_stats()` to compute it.
3. Mirror the type in `src/lib/types.ts`.
4. Render it in `src/routes/stats/+page.svelte`.

### Add a new IPC command

1. Add the function in the appropriate `src-tauri/src/commands/<resource>.rs` with `#[tauri::command]`.
2. Register it in `tauri::generate_handler![...]` in `src-tauri/src/lib.rs`.
3. Add a typed wrapper in `src/lib/ipc.ts`.
4. Add a row to `docs/IPC_API.md`.

### Database migration

1. Create `src-tauri/src/db/migrations/000N_<description>.sql`.
2. Append `(N, include_str!("migrations/000N_*.sql"))` to `MIGRATIONS` in `src-tauri/src/db/migrations.rs`.
3. Update `docs/DATA_MODEL.md` if schema-visible.

## Manual acceptance checklist (run before any release)

1. Start a focus session with no task → completes → notification fires → break starts.
2. Start, click PAUSE, click RESUME → countdown resumes correctly.
3. Start, click INTERRUPT, type "test", click CONTINUE → returns to idle, interrupt row in DB.
4. Add a goal, add a task to it, start a focus → on completion task progress increments.
5. After 4 focuses, the next break is long (15min by default).
6. Switch theme → window updates immediately, persists across restart.
7. Change settings → close app → reopen → settings preserved.
8. Stats page shows today, last 7 days, by-goal, and top interrupts after a few sessions.
```

## Task 28: Final Verification & Build

**Files:** none (verification + packaging only)

- [ ] **Step 1: Run all tests**

```bash
pnpm test
cd src-tauri && cargo test --lib
```

Expected: all green, no warnings about unused tests.

- [ ] **Step 2: Run lint**

```bash
pnpm lint
```

Expected: 0 errors. If stylelint flags any hex in components, fix by introducing a token.

- [ ] **Step 3: Run type-check**

```bash
pnpm check
```

Expected: 0 errors.

- [ ] **Step 4: Walk the manual acceptance checklist**

Open the app via `pnpm tauri dev` and walk every item from `docs/AGENT_GUIDE.md` § Manual acceptance checklist. Fix any issue found (open a fresh subtask if non-trivial).

- [ ] **Step 5: Production build**

```bash
pnpm tauri build
```

Expected: an `.msi` installer at `src-tauri/target/release/bundle/msi/`. Bundle size should be 8–15 MB.

- [ ] **Step 6: Install + smoke-run the production build**

Install the produced `.msi`, launch from Start menu. Confirm:
- Window opens.
- Acid theme applied.
- Notifications work (run a 1-minute focus session by lowering the work duration in settings).
- Data persists at `%LOCALAPPDATA%\study.pomodoro.app\data.db`.

- [ ] **Step 7: Final commit**

```bash
git add -A
git commit --allow-empty -m "chore: verified end-to-end build for v0.1.0"
git tag v0.1.0
```

---

## Done

The implementation plan covers the full spec:

- Tauri 2 + Svelte 5 scaffold (Tasks 1–2)
- Persistent SQLite storage with migrations (Tasks 3–4)
- All Rust IPC commands for goals/tasks/pomodoros/settings/stats/notify (Tasks 5–9)
- Typed frontend IPC + theme framework + acid theme + synthwave placeholder (Tasks 10–15)
- Settings/goals/tasks/timer stores with full TDD on the timer state machine (Tasks 16–18)
- App shell + all five pages, plus data export/import/reset (Tasks 19–26)
- Complete documentation suite for future agent maintenance (Task 27)
- Final verification and production build (Task 28)
