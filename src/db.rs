use rusqlite::{Connection};
use std::path::Path;

pub fn initialize_db(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS repositories (
            id INTEGER PRIMARY KEY,
            full_name TEXT UNIQUE NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS good_first_issues (
            id INTEGER PRIMARY KEY,
            issue_id INTEGER NOT NULL,
            repo_name TEXT NOT NULL,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            created_at TEXT NOT NULL,
            UNIQUE(issue_id)
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS processed_archives (
            archive_key TEXT PRIMARY KEY
        )",
        [],
    )?;
    Ok(())
}

pub fn print_issues_from_db(conn: &Connection) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare(
        "SELECT issue_id, repo_name, title, url, created_at FROM good_first_issues ORDER BY created_at DESC"
    )?;
    let issues = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
        ))
    })?;

    for issue in issues {
        let (issue_id, repo_name, title, url, created_at) = issue?;
        println!(
            "- [{}] {} ({}): {}\n  Created at: {}",
            issue_id, title, repo_name, url, created_at
        );
    }

    Ok(())
}

pub fn is_archive_processed(conn: &Connection, archive_key: &str) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM processed_archives WHERE archive_key = ?1"
    )?;
    let count: i64 = stmt.query_row([archive_key], |row| row.get(0))?;
    Ok(count > 0)
}

pub fn mark_archive_processed(conn: &Connection, archive_key: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO processed_archives (archive_key) VALUES (?1)",
        [archive_key],
    )?;
    Ok(())
}

pub fn validate_db_path(db_path: &str) -> Result<(), String> {
    if let Some(parent) = Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(format!(
                "Error: The directory for the database path '{}' does not exist.",
                db_path
            ));
        }
    }
    Ok(())
}
