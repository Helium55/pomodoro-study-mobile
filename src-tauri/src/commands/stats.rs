use crate::commands::today_local;
use crate::error::AppResult;
use crate::state::AppState;
use chrono::{Duration, Local, NaiveDate};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct StatBlock {
    pub pomos: i64,
    pub focus_secs: i64,
    pub interrupts: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DayStat {
    pub date: String,
    pub pomos: i64,
    pub focus_secs: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct GoalStat {
    pub goal_id: Option<i64>,
    pub goal_title: String,
    pub pomos: i64,
    pub focus_secs: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InterruptStat {
    pub reason: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsSummary {
    pub today: StatBlock,
    pub total: StatBlock,
    pub streak_days: i64,
    pub last_7_days: Vec<DayStat>,
    pub by_goal: Vec<GoalStat>,
    pub top_interrupts: Vec<InterruptStat>,
}

#[tauri::command]
pub fn get_stats(state: State<'_, AppState>) -> AppResult<StatsSummary> {
    let conn = state.db.lock();
    let today = today_local();

    let today_block = stat_block(
        &conn,
        "WHERE p.date_local=?1",
        &[&today as &dyn rusqlite::ToSql],
    )?;
    let total_block = stat_block(&conn, "", &[])?;
    let last_7_days = last_7_days(&conn)?;
    let streak_days = streak_days(&conn)?;
    let by_goal = by_goal(&conn)?;
    let top_interrupts = top_interrupts(&conn)?;

    Ok(StatsSummary {
        today: today_block,
        total: total_block,
        streak_days,
        last_7_days,
        by_goal,
        top_interrupts,
    })
}

fn stat_block(
    conn: &rusqlite::Connection,
    where_clause: &str,
    params: &[&dyn rusqlite::ToSql],
) -> AppResult<StatBlock> {
    let sql = format!(
        "SELECT
            COALESCE(SUM(CASE WHEN p.status='completed' THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN p.status='completed' THEN p.actual_secs ELSE 0 END), 0),
            COALESCE((SELECT count(*) FROM interrupts i JOIN pomodoros ip ON ip.id=i.pomodoro_id {}), 0)
         FROM pomodoros p {}",
        where_clause.replace("p.", "ip."),
        where_clause
    );
    let block = conn.query_row(&sql, params, |row| {
        Ok(StatBlock {
            pomos: row.get(0)?,
            focus_secs: row.get(1)?,
            interrupts: row.get(2)?,
        })
    })?;
    Ok(block)
}

fn last_7_days(conn: &rusqlite::Connection) -> AppResult<Vec<DayStat>> {
    let today = Local::now().date_naive();
    let mut days = Vec::new();
    for offset in (0..7).rev() {
        let date = (today - Duration::days(offset)).format("%Y-%m-%d").to_string();
        let (pomos, focus_secs): (i64, i64) = conn.query_row(
            "SELECT
                COALESCE(SUM(CASE WHEN status='completed' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN status='completed' THEN actual_secs ELSE 0 END), 0)
             FROM pomodoros WHERE date_local=?1",
            params![date],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        days.push(DayStat {
            date,
            pomos,
            focus_secs,
        });
    }
    Ok(days)
}

fn streak_days(conn: &rusqlite::Connection) -> AppResult<i64> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT date_local
         FROM pomodoros
         WHERE status='completed'
         ORDER BY date_local DESC",
    )?;
    let dates = stmt
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    let mut expected = Local::now().date_naive();
    let mut streak = 0;
    for date in dates {
        let parsed = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .map_err(|err| crate::error::AppError::Other(err.to_string()))?;
        if parsed == expected {
            streak += 1;
            expected -= Duration::days(1);
        } else if parsed < expected {
            break;
        }
    }
    Ok(streak)
}

fn by_goal(conn: &rusqlite::Connection) -> AppResult<Vec<GoalStat>> {
    let mut stmt = conn.prepare(
        "SELECT p.goal_id, COALESCE(g.title, 'No goal') AS goal_title,
                count(*), COALESCE(SUM(p.actual_secs), 0)
         FROM pomodoros p
         LEFT JOIN goals g ON g.id=p.goal_id
         WHERE p.status='completed'
         GROUP BY p.goal_id, goal_title
         ORDER BY count(*) DESC, goal_title ASC
         LIMIT 10",
    )?;
    let rows = stmt
        .query_map([], |row| {
            Ok(GoalStat {
                goal_id: row.get(0)?,
                goal_title: row.get(1)?,
                pomos: row.get(2)?,
                focus_secs: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

fn top_interrupts(conn: &rusqlite::Connection) -> AppResult<Vec<InterruptStat>> {
    let mut stmt = conn.prepare(
        "SELECT
            CASE
              WHEN trim(COALESCE(reason, '')) = '' THEN 'Unspecified'
              ELSE reason
            END AS label,
            count(*)
         FROM interrupts
         GROUP BY label
         ORDER BY count(*) DESC, label ASC
         LIMIT 5",
    )?;
    let rows = stmt
        .query_map([], |row| {
            Ok(InterruptStat {
                reason: row.get(0)?,
                count: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}
