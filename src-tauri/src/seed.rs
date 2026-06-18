use crate::error::AppResult;
use rusqlite::Connection;

const DEFAULTS: &[(&str, &str)] = &[
    ("timer.work_secs", "1500"),
    ("timer.break_secs", "300"),
    ("timer.long_break_secs", "900"),
    ("timer.long_break_every", "4"),
    ("timer.auto_continue", "false"),
    ("language", "\"zh-CN\""),
    ("notify.system", "true"),
    ("notify.sound", "true"),
    ("notify.sound_file", "\"ding.wav\""),
    ("notify.vibration", "true"),
    ("notify.foreground", "true"),
    ("notify.fullscreen", "true"),
    ("notify.taskbar", "true"),
    ("theme", "\"acid\""),
];

pub fn seed_defaults(conn: &Connection) -> AppResult<()> {
    for (key, value) in DEFAULTS {
        conn.execute(
            "INSERT OR IGNORE INTO settings(key, value) VALUES(?1, ?2)",
            [key, value],
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[test]
    fn seed_inserts_defaults() {
        let db = db::open_memory().unwrap();
        seed_defaults(&db.lock()).unwrap();
        let count: i64 = db
            .lock()
            .query_row("SELECT count(*) FROM settings", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 14);
    }

    #[test]
    fn seed_is_idempotent() {
        let db = db::open_memory().unwrap();
        seed_defaults(&db.lock()).unwrap();
        seed_defaults(&db.lock()).unwrap();
        let count: i64 = db
            .lock()
            .query_row("SELECT count(*) FROM settings", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 14);
    }

    #[test]
    fn seed_sets_default_language_to_simplified_chinese() {
        let db = db::open_memory().unwrap();
        seed_defaults(&db.lock()).unwrap();
        let language: String = db
            .lock()
            .query_row("SELECT value FROM settings WHERE key='language'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(language, "\"zh-CN\"");
    }

    #[test]
    fn seed_sets_default_sound_to_bundled_wav() {
        let db = db::open_memory().unwrap();
        seed_defaults(&db.lock()).unwrap();
        let sound_file: String = db
            .lock()
            .query_row(
                "SELECT value FROM settings WHERE key='notify.sound_file'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(sound_file, "\"ding.wav\"");
    }
}
