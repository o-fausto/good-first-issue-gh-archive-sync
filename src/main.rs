use chrono::{DateTime, Datelike, Timelike, Utc};
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use rusqlite::{params, Connection};
use serde_json::Value;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Connect to SQLite
    let conn = Connection::open("good_first_issues.db")?;
    initialize_db(&conn)?;

    // Step 2: Fetch the list of repositories to monitor
    let repositories: Vec<String> = conn
        .prepare("SELECT full_name FROM repositories")?
        .query_map([], |row| row.get(0))?
        .collect::<Result<_, _>>()?;

    // Step 3: Determine the most recent GitHub Archive URL
    let now: DateTime<Utc> = Utc::now();
    let hour = now.hour();
    let url = format!(
        "https://data.gharchive.org/{:04}-{:02}-{:02}-{:02}.json.gz",
        now.year(),
        now.month(),
        now.day(),
        hour
    );

    println!("Downloading file from: {}", url);

    let mut url = format!(
        "https://data.gharchive.org/{:04}-{:02}-{:02}-{:02}.json.gz",
        now.year(),
        now.month(),
        now.day(),
        hour
    );

    let mut response = get(&url)?;

    if !response.status().is_success() {
        // Retry with the previous hour's archive
        let previous_hour = if hour == 0 { 23 } else { hour - 1 };
        url = format!(
            "https://data.gharchive.org/{:04}-{:02}-{:02}-{:02}.json.gz",
            now.year(),
            now.month(),
            now.day(),
            previous_hour
        );
        println!("Retrying with previous hour: {}", url);
        response = get(&url)?;
    }

    if !response.status().is_success() {
        eprintln!("Failed to download file: HTTP {}", response.status());
        return Ok(());
    }

    println!("Decompressing and filtering issues...");
    let decoder = GzDecoder::new(response);
    let buffered = io::BufReader::new(decoder);

    for line in buffered.lines() {
        let line = line?;
        let json: Value = serde_json::from_str(&line)?;

        if let Some(repo_name) = json
            .get("repo")
            .and_then(|r| r.get("name"))
            .and_then(|r| r.as_str())
        {
            if repositories.contains(&repo_name.to_string()) {
                if let Some(issue) = json.get("payload").and_then(|payload| payload.get("issue")) {
                    if let Some(labels) = issue.get("labels") {
                        if labels
                            .as_array()
                            .map(|arr| {
                                arr.iter().any(|label| {
                                    label.get("name")
                                        == Some(&Value::String("good first issue".to_string()))
                                })
                            })
                            .unwrap_or(false)
                        {
                            let issue_id = issue.get("id").and_then(|id| id.as_i64()).unwrap_or(0);
                            let title = issue.get("title").and_then(|t| t.as_str()).unwrap_or("");
                            let url = issue.get("html_url").and_then(|u| u.as_str()).unwrap_or("");
                            let created_at = issue
                                .get("created_at")
                                .and_then(|c| c.as_str())
                                .unwrap_or("");

                            if let Err(_) = conn.execute(
                                "INSERT INTO good_first_issues (issue_id, repo_name, title, url, created_at)
                                 VALUES (?1, ?2, ?3, ?4, ?5)",
                                params![issue_id, repo_name, title, url, created_at],
                            ) {
                                println!("Issue already exists: {}", issue_id);
                            } else {
                                println!("New issue added: {} - {}", repo_name, title);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn initialize_db(conn: &Connection) -> rusqlite::Result<()> {
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
    Ok(())
}
