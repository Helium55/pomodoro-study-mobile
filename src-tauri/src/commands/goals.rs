use crate::commands::now_ms;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Goal {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub status: String,
    pub created_at: i64,
    pub archived_at: Option<i64>,
}

fn read_goal(conn: &Connection, id: i64) -> AppResult<Goal> {
    conn.query_row(
        "SELECT id, title, description, color, status, created_at, archived_at
         FROM goals WHERE id=?1",
        params![id],
        |row| {
            Ok(Goal {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                status: row.get(4)?,
                created_at: row.get(5)?,
                archived_at: row.get(6)?,
            })
        },
    )
    .map_err(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => AppError::NotFound,
        other => AppError::Db(other),
    })
}

#[tauri::command]
pub fn create_goal(
    state: State<'_, AppState>,
    title: String,
    description: Option<String>,
    color: Option<String>,
) -> AppResult<Goal> {
    let conn = state.db.lock();
    create_goal_inner(&conn, title, description, color)
}

pub fn create_goal_inner(
    conn: &Connection,
    title: String,
    description: Option<String>,
    color: Option<String>,
) -> AppResult<Goal> {
    let now = now_ms();
    conn.execute(
        "INSERT INTO goals(title, description, color, status, created_at)
         VALUES(?1, ?2, ?3, 'active', ?4)",
        params![title, description, color, now],
    )?;
    read_goal(conn, conn.last_insert_rowid())
}

#[tauri::command]
pub fn list_goals(state: State<'_, AppState>, include_archived: bool) -> AppResult<Vec<Goal>> {
    let conn = state.db.lock();
    let sql = if include_archived {
        "SELECT id, title, description, color, status, created_at, archived_at
         FROM goals ORDER BY created_at DESC, id DESC"
    } else {
        "SELECT id, title, description, color, status, created_at, archived_at
         FROM goals WHERE status='active' ORDER BY created_at DESC, id DESC"
    };
    let mut stmt = conn.prepare(sql)?;
    let goals = stmt
        .query_map([], |row| {
            Ok(Goal {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                status: row.get(4)?,
                created_at: row.get(5)?,
                archived_at: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
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
    if let Some(value) = title {
        conn.execute("UPDATE goals SET title=?1 WHERE id=?2", params![value, id])?;
    }
    if let Some(value) = description {
        conn.execute("UPDATE goals SET description=?1 WHERE id=?2", params![value, id])?;
    }
    if let Some(value) = color {
        conn.execute("UPDATE goals SET color=?1 WHERE id=?2", params![value, id])?;
    }
    Ok(())
}

#[tauri::command]
pub fn archive_goal(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.db.lock();
    let changed = conn.execute(
        "UPDATE goals SET status='archived', archived_at=?1 WHERE id=?2",
        params![now_ms(), id],
    )?;
    if changed == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

#[tauri::command]
pub fn delete_goal(state: State<'_, AppState>, id: i64) -> AppResult<()> {
    let conn = state.db.lock();
    let changed = conn.execute("DELETE FROM goals WHERE id=?1", params![id])?;
    if changed == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn create_then_read_goal() {
        let db = db::open_memory().unwrap();
        let goal = create_goal_inner(&db.lock(), "CET-6".into(), None, None).unwrap();
        assert_eq!(goal.title, "CET-6");
        assert_eq!(goal.status, "active");
    }

    #[test]
    fn archive_filters_active_goals() {
        let db = db::open_memory().unwrap();
        let goal = create_goal_inner(&db.lock(), "Math".into(), None, None).unwrap();
        db.lock()
            .execute("UPDATE goals SET status='archived' WHERE id=?1", params![goal.id])
            .unwrap();
        let active: i64 = db
            .lock()
            .query_row("SELECT count(*) FROM goals WHERE status='active'", [], |row| row.get(0))
            .unwrap();
        assert_eq!(active, 0);
    }
}
