use crate::commands::now_ms;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: i64,
    pub goal_id: Option<i64>,
    pub title: String,
    pub estimated_pomos: i64,
    pub status: String,
    pub created_at: i64,
    pub done_at: Option<i64>,
    pub sort_order: i64,
    pub completed_pomos: i64,
}

fn task_select_clause() -> &'static str {
    "SELECT t.id, t.goal_id, t.title, t.estimated_pomos, t.status, t.created_at, t.done_at,
            t.sort_order,
            COALESCE((
              SELECT count(*) FROM pomodoros p
              WHERE p.task_id=t.id AND p.status='completed'
            ), 0) AS completed_pomos
     FROM tasks t"
}

fn map_task(row: &rusqlite::Row<'_>) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        goal_id: row.get(1)?,
        title: row.get(2)?,
        estimated_pomos: row.get(3)?,
        status: row.get(4)?,
        created_at: row.get(5)?,
        done_at: row.get(6)?,
        sort_order: row.get(7)?,
        completed_pomos: row.get(8)?,
    })
}

fn read_task(conn: &Connection, id: i64) -> AppResult<Task> {
    conn.query_row(
        &format!("{} WHERE t.id=?1", task_select_clause()),
        params![id],
        map_task,
    )
    .map_err(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => AppError::NotFound,
        other => AppError::Db(other),
    })
}

#[tauri::command]
pub fn create_task(
    state: State<'_, AppState>,
    goal_id: Option<i64>,
    title: String,
    estimated_pomos: Option<i64>,
) -> AppResult<Task> {
    let conn = state.db.lock();
    create_task_inner(&conn, goal_id, title, estimated_pomos.unwrap_or(1))
}

pub fn create_task_inner(
    conn: &Connection,
    goal_id: Option<i64>,
    title: String,
    estimated_pomos: i64,
) -> AppResult<Task> {
    let next_order: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) + 1 FROM tasks
             WHERE (goal_id IS ?1) OR (goal_id=?1)",
            params![goal_id],
            |row| row.get(0),
        )
        .unwrap_or(0);
    conn.execute(
        "INSERT INTO tasks(goal_id, title, estimated_pomos, status, created_at, sort_order)
         VALUES(?1, ?2, ?3, 'active', ?4, ?5)",
        params![goal_id, title, estimated_pomos.max(1), now_ms(), next_order],
    )?;
    read_task(conn, conn.last_insert_rowid())
}

#[tauri::command]
pub fn list_tasks(
    state: State<'_, AppState>,
    goal_id: Option<i64>,
    include_done: bool,
) -> AppResult<Vec<Task>> {
    let conn = state.db.lock();
    list_tasks_inner(&conn, goal_id, include_done)
}

pub fn list_tasks_inner(
    conn: &Connection,
    goal_id: Option<i64>,
    include_done: bool,
) -> AppResult<Vec<Task>> {
    let mut sql = String::from(task_select_clause());
    match (goal_id, include_done) {
        (Some(_), true) => sql.push_str(" WHERE t.goal_id=?1"),
        (Some(_), false) => sql.push_str(" WHERE t.goal_id=?1 AND t.status='active'"),
        (None, true) => sql.push_str(" WHERE (?1 IS NULL OR t.goal_id=?1)"),
        (None, false) => sql.push_str(" WHERE (?1 IS NULL OR t.goal_id=?1) AND t.status='active'"),
    }
    sql.push_str(" ORDER BY t.sort_order ASC, t.created_at ASC, t.id ASC");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map(params![goal_id], map_task)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

#[tauri::command]
pub fn update_task(
    state: State<'_, AppState>,
    id: i64,
    title: Option<String>,
    goal_id: Option<i64>,
    estimated_pomos: Option<i64>,
) -> AppResult<()> {
    let conn = state.db.lock();
    if let Some(value) = title {
        conn.execute("UPDATE tasks SET title=?1 WHERE id=?2", params![value, id])?;
    }
    if let Some(value) = goal_id {
        conn.execute("UPDATE tasks SET goal_id=?1 WHERE id=?2", params![value, id])?;
    }
    if let Some(value) = estimated_pomos {
        conn.execute(
            "UPDATE tasks SET estimated_pomos=?1 WHERE id=?2",
            params![value.max(1), id],
        )?;
    }
    Ok(())
}

#[tauri::command]
pub fn complete_task(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.db.lock();
    let changed = conn.execute(
        "UPDATE tasks SET status='done', done_at=?1 WHERE id=?2",
        params![now_ms(), id],
    )?;
    if changed == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

#[tauri::command]
pub fn delete_task(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.db.lock();
    let changed = conn.execute("DELETE FROM tasks WHERE id=?1", params![id])?;
    if changed == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

#[tauri::command]
pub fn reorder_tasks(state: State<'_, AppState>, ordered_ids: Vec<i64>) -> AppResult<()> {
    let mut conn = state.db.lock();
    let tx = conn.transaction()?;
    for (index, id) in ordered_ids.iter().enumerate() {
        tx.execute(
            "UPDATE tasks SET sort_order=?1 WHERE id=?2",
            params![index as i64, id],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub fn goal_for_task(conn: &Connection, task_id: i64) -> AppResult<Option<i64>> {
    conn.query_row(
        "SELECT goal_id FROM tasks WHERE id=?1",
        params![task_id],
        |row| row.get(0),
    )
    .optional()?
    .ok_or(AppError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn create_task_assigns_order() {
        let db = db::open_memory().unwrap();
        let first = create_task_inner(&db.lock(), None, "Read".into(), 2).unwrap();
        let second = create_task_inner(&db.lock(), None, "Review".into(), 1).unwrap();
        assert_eq!(first.sort_order, 0);
        assert_eq!(second.sort_order, 1);
    }

    #[test]
    fn list_hides_done_by_default() {
        let db = db::open_memory().unwrap();
        let task = create_task_inner(&db.lock(), None, "Done".into(), 1).unwrap();
        db.lock()
            .execute("UPDATE tasks SET status='done' WHERE id=?1", params![task.id])
            .unwrap();
        let tasks = list_tasks_inner(&db.lock(), None, false).unwrap();
        assert!(tasks.is_empty());
    }
}
