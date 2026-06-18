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
  id           INTEGER PRIMARY KEY,
  pomodoro_id  INTEGER NOT NULL REFERENCES pomodoros(id) ON DELETE CASCADE,
  reason       TEXT,
  occurred_at  INTEGER NOT NULL
);

CREATE TABLE settings (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

CREATE INDEX idx_pomos_date ON pomodoros(date_local);
CREATE INDEX idx_pomos_task ON pomodoros(task_id);
CREATE INDEX idx_pomos_goal ON pomodoros(goal_id);
CREATE INDEX idx_tasks_goal ON tasks(goal_id);
