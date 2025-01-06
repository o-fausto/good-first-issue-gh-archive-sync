// use flate2::read::GzDecoder;
// use reqwest::blocking::get;
// use rusqlite::{params, Connection};
// use serde_json::Value;
// use std::io::{self, BufRead};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Step 1: Connect to SQLite
//     let conn = Connection::open("good_first_issues.db")?;
//     initialize_db(&conn)?;

//     // Step 2: Fetch the list of repositories to monitor
//     let repositories: Vec<String> = conn
//         .prepare("SELECT full_name FROM repositories")?
//         .query_map([], |row| row.get(0))?
//         .collect::<Result<_, _>>()?;

//     // Step 3: Download and process the latest GitHub Archive data
//     let url = "https://data.gharchive.org/2025-01-05-0.json.gz";
//     println!("Downloading file from: {}", url);
//     let response = get(url)?;
//     if !response.status().is_success() {
//         eprintln!("Failed to download file: HTTP {}", response.status());
//         return Ok(());
//     }

//     println!("Decompressing and filtering issues...");
//     let decoder = GzDecoder::new(response);
//     let buffered = io::BufReader::new(decoder);

//     for line in buffered.lines() {
//         let line = line?;
//         let json: Value = serde_json::from_str(&line)?;

//         // println!("{:#?}", json);

//         // Check if the event is an issue or pull request with the desired label.
//         if let Some(repo_name) = json
//             .get("repo")
//             .and_then(|r| r.get("name"))
//             .and_then(|r| r.as_str())
//         {
//             if repositories.contains(&repo_name.to_string()) {
//                 if let Some(issue) = json.get("payload").and_then(|payload| payload.get("issue")) {
//                     if let Some(labels) = issue.get("labels") {
//                         if labels
//                             .as_array()
//                             .map(|arr| {
//                                 arr.iter().any(|label| {
//                                     label.get("name")
//                                         == Some(&Value::String("good first issue".to_string()))
//                                 })
//                             })
//                             .unwrap_or(false)
//                         {
//                             let issue_id = issue.get("id").and_then(|id| id.as_i64()).unwrap_or(0);
//                             let title = issue.get("title").and_then(|t| t.as_str()).unwrap_or("");
//                             let url = issue.get("html_url").and_then(|u| u.as_str()).unwrap_or("");
//                             let created_at = issue
//                                 .get("created_at")
//                                 .and_then(|c| c.as_str())
//                                 .unwrap_or("");

//                             // Save the issue if it's new
//                             if let Err(_) = conn.execute(
//                                 "INSERT INTO good_first_issues (issue_id, repo_name, title, url, created_at)
//                                  VALUES (?1, ?2, ?3, ?4, ?5)",
//                                 params![issue_id, repo_name, title, url, created_at],
//                             ) {
//                                 println!("Issue already exists: {}", issue_id);
//                             } else {
//                                 println!("New issue added: {} - {}", repo_name, title);
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     Ok(())
// }

// fn initialize_db(conn: &Connection) -> rusqlite::Result<()> {
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS repositories (
//             id INTEGER PRIMARY KEY,
//             full_name TEXT UNIQUE NOT NULL
//         )",
//         [],
//     )?;
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS good_first_issues (
//             id INTEGER PRIMARY KEY,
//             issue_id INTEGER NOT NULL,
//             repo_name TEXT NOT NULL,
//             title TEXT NOT NULL,
//             url TEXT NOT NULL,
//             created_at TEXT NOT NULL,
//             UNIQUE(issue_id)
//         )",
//         [],
//     )?;
//     Ok(())
// }

// use flate2::read::GzDecoder;
// use reqwest::blocking::get;
// use rusqlite::{params, Connection};
// use serde_json::Value;
// use std::io::{self, BufRead};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Step 1: Connect to SQLite
//     let conn = Connection::open("good_first_issues.db")?;
//     initialize_db(&conn)?;

//     // Step 2: Fetch the list of repositories to monitor
//     let repositories: Vec<String> = conn
//         .prepare("SELECT full_name FROM repositories")?
//         .query_map([], |row| row.get(0))?
//         .collect::<Result<_, _>>()?;

//     // Step 3: Download and process the latest GitHub Archive data
//     let url = "https://data.gharchive.org/2025-01-05-0.json.gz";
//     println!("Downloading file from: {}", url);
//     let response = get(url)?;
//     if !response.status().is_success() {
//         eprintln!("Failed to download file: HTTP {}", response.status());
//         return Ok(());
//     }

//     println!("Decompressing and processing events...");
//     let decoder = GzDecoder::new(response);
//     let buffered = io::BufReader::new(decoder);

//     for line in buffered.lines() {
//         let line = line?;
//         let json: Value = serde_json::from_str(&line)?;

//         // println!("{:#?}", json);

//         // Process only "issues" events
//         if let Some(event_type) = json.get("type").and_then(|t| t.as_str()) {
//             if event_type == "IssuesEvent" {
//                 process_issues_event(&conn, &repositories, &json)?;
//             }
//         }
//     }

//     Ok(())
// }

// fn process_issues_event(
//     conn: &Connection,
//     repositories: &[String],
//     event: &Value,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     if let Some(repo_name) = event
//         .get("repo")
//         .and_then(|r| r.get("name"))
//         .and_then(|r| r.as_str())
//     {
//         println!("Event repo name: {}", repo_name);
//         println!("Monitored repositories: {:?}", repositories);
//         // println!("Processing repository: {}", repo_name);
//         if repositories.contains(&repo_name.to_string()) {
//             println!("Repository is being monitored: {}", repo_name);

//             if let Some(action) = event
//                 .get("payload")
//                 .and_then(|p| p.get("action"))
//                 .and_then(|a| a.as_str())
//             {
//                 let issue = event
//                     .get("payload")
//                     .and_then(|p| p.get("issue"))
//                     .unwrap_or(&serde_json::Value::Null);
//                 let issue_id = issue.get("id").and_then(|id| id.as_i64()).unwrap_or(0);

//                 println!("Action: {}", action);
//                 match action {
//                     "labeled" => {
//                         if is_good_first_issue_label(issue.get("labels")) {
//                             let title = issue.get("title").and_then(|t| t.as_str()).unwrap_or("");
//                             let url = issue.get("html_url").and_then(|u| u.as_str()).unwrap_or("");
//                             let created_at = issue
//                                 .get("created_at")
//                                 .and_then(|c| c.as_str())
//                                 .unwrap_or("");

//                             println!("Adding issue: {} - {}", title, url);
//                             if let Err(err) = conn.execute(
//                                 "INSERT INTO good_first_issues (issue_id, repo_name, title, url, created_at)
//                                  VALUES (?1, ?2, ?3, ?4, ?5)",
//                                 params![issue_id, repo_name, title, url, created_at],
//                             ) {
//                                 println!("Failed to add issue: {}", err);
//                             } else {
//                                 println!("Issue added successfully: {}", issue_id);
//                             }
//                         }
//                     }
//                     "unlabeled" => {
//                         if let Some(label) = event
//                             .get("payload")
//                             .and_then(|p| p.get("label"))
//                             .and_then(|l| l.get("name"))
//                             .and_then(|l| l.as_str())
//                         {
//                             if label == "good first issue" {
//                                 println!("Removing issue due to label removal: {}", issue_id);
//                                 conn.execute(
//                                     "DELETE FROM good_first_issues WHERE issue_id = ?1",
//                                     params![issue_id],
//                                 )?;
//                             }
//                         }
//                     }
//                     "closed" => {
//                         println!("Removing issue due to closure: {}", issue_id);
//                         conn.execute(
//                             "DELETE FROM good_first_issues WHERE issue_id = ?1",
//                             params![issue_id],
//                         )?;
//                     }
//                     _ => {}
//                 }
//             }
//         } else {
//             // println!("Repository is NOT being monitored: {}", repo_name);
//         }
//     }

//     Ok(())
// }

// fn is_good_first_issue_label(labels: Option<&Value>) -> bool {
//     if let Some(labels) = labels {
//         if let Some(array) = labels.as_array() {
//             return array.iter().any(|label| {
//                 label.get("name") == Some(&Value::String("good first issue".to_string()))
//             });
//         }
//     }
//     false
// }

// fn initialize_db(conn: &Connection) -> rusqlite::Result<()> {
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS repositories (
//             id INTEGER PRIMARY KEY,
//             full_name TEXT UNIQUE NOT NULL
//         )",
//         [],
//     )?;
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS good_first_issues (
//             id INTEGER PRIMARY KEY,
//             issue_id INTEGER NOT NULL,
//             repo_name TEXT NOT NULL,
//             title TEXT NOT NULL,
//             url TEXT NOT NULL,
//             created_at TEXT NOT NULL,
//             UNIQUE(issue_id)
//         )",
//         [],
//     )?;
//     Ok(())
// }

// use flate2::read::GzDecoder;
// use reqwest::blocking::get;
// use rusqlite::{params, Connection};
// use serde_json::Value;
// use std::collections::HashSet;
// use std::io::{self, BufRead};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Step 1: Connect to SQLite
//     let conn = Connection::open("good_first_issues.db")?;
//     initialize_db(&conn)?;

//     // Step 2: Fetch the list of repositories to monitor
//     let repositories: Vec<String> = conn
//         .prepare("SELECT full_name FROM repositories")?
//         .query_map([], |row| row.get(0))?
//         .collect::<Result<_, _>>()?;

//     // Step 3: Download and process the latest GitHub Archive data
//     let url = "https://data.gharchive.org/2025-01-05-0.json.gz";
//     println!("Downloading file from: {}", url);
//     let response = get(url)?;
//     if !response.status().is_success() {
//         eprintln!("Failed to download file: HTTP {}", response.status());
//         return Ok(());
//     }

//     println!("Decompressing and processing events...");
//     let decoder = GzDecoder::new(response);
//     let buffered = io::BufReader::new(decoder);

//     // Track active issues with "good first issue" label
//     let mut active_issues: HashSet<i64> = HashSet::new();

//     for line in buffered.lines() {
//         let line = line?;
//         let event: Value = serde_json::from_str(&line)?;

//         if let Some(repo_name) = event
//             .get("repo")
//             .and_then(|r| r.get("name"))
//             .and_then(|r| r.as_str())
//         {
//             if repositories.contains(&repo_name.to_string()) {
//                 process_event(&conn, repo_name, &event, &mut active_issues)?;
//             }
//         }
//     }

//     // Remove issues that are no longer active
//     cleanup_issues(&conn, active_issues)?;

//     Ok(())
// }

// fn initialize_db(conn: &Connection) -> rusqlite::Result<()> {
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS repositories (
//             id INTEGER PRIMARY KEY,
//             full_name TEXT UNIQUE NOT NULL
//         )",
//         [],
//     )?;
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS good_first_issues (
//             id INTEGER PRIMARY KEY,
//             issue_id INTEGER NOT NULL,
//             repo_name TEXT NOT NULL,
//             title TEXT NOT NULL,
//             url TEXT NOT NULL,
//             created_at TEXT NOT NULL,
//             UNIQUE(issue_id)
//         )",
//         [],
//     )?;
//     Ok(())
// }

// fn process_event(
//     conn: &Connection,
//     repo_name: &str,
//     event: &Value,
//     active_issues: &mut HashSet<i64>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     if let Some(action) = event
//         .get("payload")
//         .and_then(|p| p.get("action"))
//         .and_then(|a| a.as_str())
//     {
//         if action == "closed" {
//             // Handle closed issues
//             if let Some(issue_id) = event
//                 .get("payload")
//                 .and_then(|p| p.get("issue"))
//                 .and_then(|i| i.get("id"))
//                 .and_then(|id| id.as_i64())
//             {
//                 conn.execute(
//                     "DELETE FROM good_first_issues WHERE issue_id = ?",
//                     params![issue_id],
//                 )?;
//                 println!("Issue closed and removed: {}", issue_id);
//             }
//         } else if action == "labeled" || action == "unlabeled" || action == "opened" {
//             // Handle labeled/unlabeled/opened events
//             if let Some(issue) = event.get("payload").and_then(|p| p.get("issue")) {
//                 let issue_id = issue.get("id").and_then(|id| id.as_i64()).unwrap_or(0);
//                 if action == "unlabeled" {
//                     if let Some(label) = event
//                         .get("payload")
//                         .and_then(|p| p.get("label"))
//                         .and_then(|l| l.get("name"))
//                         .and_then(|n| n.as_str())
//                     {
//                         if label == "good first issue" {
//                             conn.execute(
//                                 "DELETE FROM good_first_issues WHERE issue_id = ?",
//                                 params![issue_id],
//                             )?;
//                             println!("Issue untagged and removed: {}", issue_id);
//                         }
//                     }
//                 } else if action == "labeled" || action == "opened" {
//                     if let Some(labels) = issue.get("labels") {
//                         if labels.as_array().map_or(false, |arr| {
//                             arr.iter().any(|label| {
//                                 label.get("name")
//                                     == Some(&Value::String("good first issue".to_string()))
//                             })
//                         }) {
//                             let title = issue.get("title").and_then(|t| t.as_str()).unwrap_or("");
//                             let url = issue.get("html_url").and_then(|u| u.as_str()).unwrap_or("");
//                             let created_at = issue
//                                 .get("created_at")
//                                 .and_then(|c| c.as_str())
//                                 .unwrap_or("");

//                             conn.execute(
//                                 "INSERT INTO good_first_issues (issue_id, repo_name, title, url, created_at)
//                                  VALUES (?1, ?2, ?3, ?4, ?5)
//                                  ON CONFLICT(issue_id) DO NOTHING",
//                                 params![issue_id, repo_name, title, url, created_at],
//                             )?;
//                             println!("New or updated issue: {} - {}", repo_name, title);

//                             // Mark issue as active
//                             active_issues.insert(issue_id);
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     Ok(())
// }

// fn cleanup_issues(conn: &Connection, active_issues: HashSet<i64>) -> rusqlite::Result<()> {
//     // Fetch all currently stored issues
//     let mut stmt = conn.prepare("SELECT issue_id FROM good_first_issues")?;
//     let stored_issues: Vec<i64> = stmt
//         .query_map([], |row| row.get(0))?
//         .collect::<Result<_, _>>()?;

//     for issue_id in stored_issues {
//         if !active_issues.contains(&issue_id) {
//             conn.execute(
//                 "DELETE FROM good_first_issues WHERE issue_id = ?",
//                 params![issue_id],
//             )?;
//             println!("Removed stale issue: {}", issue_id);
//         }
//     }

//     Ok(())
// }

///////////////
///
///
///
///
// use flate2::read::GzDecoder;
// use reqwest::blocking::get;
// use rusqlite::{params, Connection};
// use serde_json::Value;
// use std::collections::HashMap;
// use std::io::{self, BufRead};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let conn = Connection::open("good_first_issues.db")?;
//     initialize_db(&conn)?;

//     let repositories: Vec<String> = conn
//         .prepare("SELECT full_name FROM repositories")?
//         .query_map([], |row| row.get(0))?
//         .collect::<Result<_, _>>()?;

//     let url = "https://data.gharchive.org/2025-01-05-0.json.gz";
//     println!("Downloading file from: {}", url);
//     let response = get(url)?;
//     if !response.status().is_success() {
//         eprintln!("Failed to download file: HTTP {}", response.status());
//         return Ok(());
//     }

//     println!("Decompressing and filtering issues...");
//     let decoder = GzDecoder::new(response);
//     let buffered = io::BufReader::new(decoder);

//     // Map to track state of monitored issues
//     let mut issue_updates: HashMap<i64, (bool, bool)> = HashMap::new();
//     // Format: issue_id -> (has_good_first_issue_label, is_open)

//     for line in buffered.lines() {
//         let line = line?;
//         let json: Value = serde_json::from_str(&line)?;

//         if let Some(repo_name) = json
//             .get("repo")
//             .and_then(|r| r.get("name"))
//             .and_then(|r| r.as_str())
//         {
//             if repositories.contains(&repo_name.to_string()) {
//                 if let Some(issue) = json.get("payload").and_then(|payload| payload.get("issue")) {
//                     let issue_id = issue.get("id").and_then(|id| id.as_i64()).unwrap_or(0);

//                     let has_label = issue
//                         .get("labels")
//                         .and_then(|labels| labels.as_array())
//                         .map(|arr| {
//                             arr.iter().any(|label| {
//                                 label.get("name")
//                                     == Some(&Value::String("good first issue".to_string()))
//                             })
//                         })
//                         .unwrap_or(false);

//                     let is_open = issue
//                         .get("state")
//                         .and_then(|state| state.as_str())
//                         .map(|state| state == "open")
//                         .unwrap_or(false);

//                     issue_updates.insert(issue_id, (has_label, is_open));

//                     // Add or update the issue if it matches criteria
//                     if has_label && is_open {
//                         let title = issue.get("title").and_then(|t| t.as_str()).unwrap_or("");
//                         let url = issue.get("html_url").and_then(|u| u.as_str()).unwrap_or("");
//                         let created_at = issue
//                             .get("created_at")
//                             .and_then(|c| c.as_str())
//                             .unwrap_or("");

//                         if let Err(_) = conn.execute(
//                             "INSERT INTO good_first_issues (issue_id, repo_name, title, url, created_at)
//                              VALUES (?1, ?2, ?3, ?4, ?5)
//                              ON CONFLICT(issue_id) DO UPDATE SET title = excluded.title, url = excluded.url",
//                             params![issue_id, repo_name, title, url, created_at],
//                         ) {
//                             println!("Issue already exists: {}", issue_id);
//                         } else {
//                             println!("New issue added or updated: {} - {}", repo_name, title);
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     // Validate stored issues against the updates
//     let mut stmt = conn.prepare("SELECT issue_id FROM good_first_issues")?;
//     let db_issue_ids: Vec<i64> = stmt
//         .query_map([], |row| row.get(0))?
//         .collect::<Result<_, _>>()?;

//     for issue_id in db_issue_ids {
//         if let Some(&(has_label, is_open)) = issue_updates.get(&issue_id) {
//             if !has_label || !is_open {
//                 conn.execute(
//                     "DELETE FROM good_first_issues WHERE issue_id = ?1",
//                     params![issue_id],
//                 )?;
//                 println!("Removed outdated issue: {}", issue_id);
//             }
//         } else {
//             // If the issue doesn't appear in the current archive, keep it unchanged
//             println!("Issue not found in archive but retained: {}", issue_id);
//         }
//     }

//     Ok(())
// }

// fn initialize_db(conn: &Connection) -> rusqlite::Result<()> {
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS repositories (
//             id INTEGER PRIMARY KEY,
//             full_name TEXT UNIQUE NOT NULL
//         )",
//         [],
//     )?;
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS good_first_issues (
//             id INTEGER PRIMARY KEY,
//             issue_id INTEGER NOT NULL,
//             repo_name TEXT NOT NULL,
//             title TEXT NOT NULL,
//             url TEXT NOT NULL,
//             created_at TEXT NOT NULL,
//             UNIQUE(issue_id)
//         )",
//         [],
//     )?;
//     Ok(())
// }
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
