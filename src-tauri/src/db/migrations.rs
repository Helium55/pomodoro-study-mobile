use anyhow::Result;
use rusqlite::Connection;

const MIGRATIONS: &[(i64, &str)] = &[
    (1, include_str!("migrations/0001_init.sql")),
    (
        2,
        include_str!("migrations/0002_settings_language_sound.sql"),
    ),
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
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    for (version, sql) in MIGRATIONS {
        if *version > current {
            conn.execute_batch(sql)?;
            conn.execute(
                "INSERT INTO schema_version(version, applied_at)
                 VALUES(?1, strftime('%s', 'now') * 1000)",
                [version],
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_updates_legacy_sound_and_adds_language() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE schema_version (
                version INTEGER PRIMARY KEY,
                applied_at INTEGER NOT NULL
            );
            INSERT INTO schema_version(version, applied_at) VALUES(1, 0);
            CREATE TABLE settings (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            INSERT INTO settings(key, value) VALUES('notify.sound_file', '"ding.mp3"');
            "#,
        )
        .unwrap();

        run(&conn).unwrap();

        let sound_file: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key='notify.sound_file'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let language: String = conn
            .query_row("SELECT value FROM settings WHERE key='language'", [], |r| {
                r.get(0)
            })
            .unwrap();

        assert_eq!(sound_file, "\"ding.wav\"");
        assert_eq!(language, "\"zh-CN\"");
    }
}
