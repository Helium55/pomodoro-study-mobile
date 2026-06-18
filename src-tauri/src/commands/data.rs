use crate::error::{AppError, AppResult};
use crate::seed;
use crate::state::AppState;
use rusqlite::types::{Value as SqlValue, ValueRef};
use rusqlite::{params_from_iter, Connection};
use serde_json::{json, Map, Value};
use tauri::State;

const TABLES: &[(&str, &[&str])] = &[
    (
        "goals",
        &["id", "title", "description", "color", "status", "created_at", "archived_at"],
    ),
    (
        "tasks",
        &[
            "id",
            "goal_id",
            "title",
            "estimated_pomos",
            "status",
            "created_at",
            "done_at",
            "sort_order",
        ],
    ),
    (
        "pomodoros",
        &[
            "id",
            "task_id",
            "goal_id",
            "started_at",
            "ended_at",
            "planned_secs",
            "actual_secs",
            "status",
            "date_local",
        ],
    ),
    ("interrupts", &["id", "pomodoro_id", "reason", "occurred_at"]),
    ("settings", &["key", "value"]),
];

#[tauri::command]
pub fn export_data(state: State<'_, AppState>) -> AppResult<String> {
    let conn = state.db.lock();
    let mut root = Map::new();
    for (table, columns) in TABLES {
        root.insert((*table).to_string(), Value::Array(export_table(&conn, table, columns)?));
    }
    serde_json::to_string_pretty(&Value::Object(root)).map_err(Into::into)
}

#[tauri::command]
pub fn import_data(state: State<'_, AppState>, json_text: String) -> AppResult<()> {
    let parsed: Value = serde_json::from_str(&json_text)?;
    let root = parsed
        .as_object()
        .ok_or_else(|| AppError::Other("import JSON must be an object".into()))?;
    let mut conn = state.db.lock();
    let tx = conn.transaction()?;
    clear_tables(&tx)?;
    for (table, columns) in TABLES {
        let rows = root.get(*table).and_then(Value::as_array).cloned().unwrap_or_default();
        import_table(&tx, table, columns, &rows)?;
    }
    tx.commit()?;
    Ok(())
}

#[tauri::command]
pub fn reset_data(state: State<'_, AppState>) -> AppResult<()> {
    let mut conn = state.db.lock();
    let tx = conn.transaction()?;
    clear_tables(&tx)?;
    tx.commit()?;
    seed::seed_defaults(&conn)?;
    Ok(())
}

fn export_table(conn: &Connection, table: &str, columns: &[&str]) -> AppResult<Vec<Value>> {
    let sql = format!("SELECT {} FROM {}", columns.join(", "), table);
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |row| {
        let mut object = Map::new();
        for (index, column) in columns.iter().enumerate() {
            object.insert((*column).to_string(), sql_value_to_json(row.get_ref(index)?));
        }
        Ok(Value::Object(object))
    })?;
    let out = rows.collect::<Result<Vec<_>, _>>()?;
    Ok(out)
}

fn sql_value_to_json(value: ValueRef<'_>) -> Value {
    match value {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(value) => json!(value),
        ValueRef::Real(value) => json!(value),
        ValueRef::Text(value) => json!(String::from_utf8_lossy(value).to_string()),
        ValueRef::Blob(value) => json!(String::from_utf8_lossy(value).to_string()),
    }
}

fn import_table(
    conn: &Connection,
    table: &str,
    columns: &[&str],
    rows: &[Value],
) -> AppResult<()> {
    if rows.is_empty() {
        return Ok(());
    }
    let placeholders = std::iter::repeat("?")
        .take(columns.len())
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "INSERT INTO {}({}) VALUES({})",
        table,
        columns.join(", "),
        placeholders
    );
    let mut stmt = conn.prepare(&sql)?;
    for row in rows {
        let object = row
            .as_object()
            .ok_or_else(|| AppError::Other(format!("{table} row must be an object")))?;
        let values = columns
            .iter()
            .map(|column| json_to_sql(object.get(*column).unwrap_or(&Value::Null)))
            .collect::<Vec<_>>();
        stmt.execute(params_from_iter(values))?;
    }
    Ok(())
}

fn json_to_sql(value: &Value) -> SqlValue {
    match value {
        Value::Null => SqlValue::Null,
        Value::Bool(value) => SqlValue::Text(value.to_string()),
        Value::Number(value) => {
            if let Some(integer) = value.as_i64() {
                SqlValue::Integer(integer)
            } else if let Some(float) = value.as_f64() {
                SqlValue::Real(float)
            } else {
                SqlValue::Null
            }
        }
        Value::String(value) => SqlValue::Text(value.clone()),
        other => SqlValue::Text(other.to_string()),
    }
}

fn clear_tables(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        "DELETE FROM interrupts;
         DELETE FROM pomodoros;
         DELETE FROM tasks;
         DELETE FROM goals;
         DELETE FROM settings;",
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::goals::create_goal_inner;
    use crate::db;

    #[test]
    fn export_includes_goals() {
        let db = db::open_memory().unwrap();
        create_goal_inner(&db.lock(), "Math".into(), None, None).unwrap();
        let value = export_table(&db.lock(), "goals", TABLES[0].1).unwrap();
        assert_eq!(value.len(), 1);
    }
}
