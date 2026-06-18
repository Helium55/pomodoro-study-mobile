use crate::commands::{now_ms, today_local};
use crate::commands::tasks::goal_for_task;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

fn map_pomodoro(row: &rusqlite::Row<'_>) -> rusqlite::Result<Pomodoro> {
    Ok(Pomodoro {
        id: row.get(0)?,
        task_id: row.get(1)?,
        goal_id: row.get(2)?,
        started_at: row.get(3)?,
        ended_at: row.get(4)?,
        planned_secs: row.get(5)?,
        actual_secs: row.get(6)?,
        status: row.get(7)?,
        date_local: row.get(8)?,
    })
}

fn read_pomodoro(conn: &Connection, id: i64) -> AppResult<Pomodoro> {
    conn.query_row(
        "SELECT id, task_id, goal_id, started_at, ended_at, planned_secs, actual_secs,
                status, date_local
         FROM pomodoros WHERE id=?1",
        params![id],
        map_pomodoro,
    )
    .map_err(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => AppError::NotFound,
        other => AppError::Db(other),
    })
}

#[tauri::command]
pub fn start_pomodoro(
    state: State<'_, AppState>,
    task_id: Option<i64>,
    planned_secs: i64,
) -> AppResult<Pomodoro> {
    let conn = state.db.lock();
    start_pomodoro_inner(&conn, task_id, planned_secs)
}

pub fn start_pomodoro_inner(
    conn: &Connection,
    task_id: Option<i64>,
    planned_secs: i64,
) -> AppResult<Pomodoro> {
    let goal_id = match task_id {
        Some(id) => goal_for_task(conn, id)?,
        None => None,
    };
    conn.execute(
        "INSERT INTO pomodoros(task_id, goal_id, started_at, planned_secs, status, date_local)
         VALUES(?1, ?2, ?3, ?4, 'in_progress', ?5)",
        params![task_id, goal_id, now_ms(), planned_secs.max(1), today_local()],
    )?;
    read_pomodoro(conn, conn.last_insert_rowid())
}

#[tauri::command]
pub fn complete_pomodoro(state: State<'_, AppState>, id: i64, actual_secs: i64) -> AppResult<()> {
    let conn = state.db.lock();
    let changed = conn.execute(
        "UPDATE pomodoros
         SET ended_at=?1, actual_secs=?2, status='completed'
         WHERE id=?3 AND status='in_progress'",
        params![now_ms(), actual_secs.max(0), id],
    )?;
    if changed == 0 {
        return Err(AppError::NotFound);
    }
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
    let status = if abandoned { "abandoned" } else { "interrupted" };
    let changed = tx.execute(
        "UPDATE pomodoros
         SET ended_at=?1, actual_secs=?2, status=?3
         WHERE id=?4 AND status='in_progress'",
        params![now_ms(), actual_secs.max(0), status, id],
    )?;
    if changed == 0 {
        return Err(AppError::NotFound);
    }
    tx.execute(
        "INSERT INTO interrupts(pomodoro_id, reason, occurred_at) VALUES(?1, ?2, ?3)",
        params![id, reason, now_ms()],
    )?;
    tx.commit()?;
    Ok(())
}

#[tauri::command]
pub fn list_pomodoros_today(state: State<'_, AppState>) -> AppResult<Vec<Pomodoro>> {
    let conn = state.db.lock();
    let mut stmt = conn.prepare(
        "SELECT id, task_id, goal_id, started_at, ended_at, planned_secs, actual_secs,
                status, date_local
         FROM pomodoros
         WHERE date_local=?1
         ORDER BY started_at DESC, id DESC",
    )?;
    let rows = stmt
        .query_map(params![today_local()], map_pomodoro)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn start_creates_in_progress_record() {
        let db = db::open_memory().unwrap();
        let pomo = start_pomodoro_inner(&db.lock(), None, 1500).unwrap();
        assert_eq!(pomo.status, "in_progress");
        assert_eq!(pomo.planned_secs, 1500);
    }
}
