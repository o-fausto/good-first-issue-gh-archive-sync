use rusqlite::{params, Connection};
use serde_json::Value;
use std::io::BufRead;

/// Returns true if the JSON event is an IssuesEvent with a "good first issue" label.
pub fn is_good_first_issue(json: &Value) -> bool {
    if json.get("type") == Some(&Value::String("IssuesEvent".to_string())) {
        if let Some(issue) = json.get("payload").and_then(|payload| payload.get("issue")) {
            if let Some(labels) = issue.get("labels").and_then(|l| l.as_array()) {
                return labels.iter().any(|label| {
                    label.get("name") == Some(&Value::String("good first issue".to_string()))
                });
            }
        }
    }
    false
}

/// Returns true if the title should be skipped (contains "test", "tests", or "fix", case-insensitive).
pub fn should_skip_title(title: &str) -> bool {
    let title_lower = title.to_lowercase();
    title_lower.contains("test") || title_lower.contains("tests") || title_lower.contains("fix")
}

/// Processes lines from a BufRead, filters, and inserts good first issues into the DB.
pub fn process_issues_from_reader<R: BufRead>(
    conn: &Connection,
    buffered: R,
) -> Result<(), Box<dyn std::error::Error>> {
    for line in buffered.lines() {
        let line = line?;
        let json: Value = match serde_json::from_str(&line) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                continue;
            }
        };

        if super::filter::is_good_first_issue(&json) {
            if let Some(repo_name) = json
                .get("repo")
                .and_then(|r| r.get("name"))
                .and_then(|r| r.as_str())
            {
                if let Some(issue) = json.get("payload").and_then(|payload| payload.get("issue")) {
                    let title = issue.get("title").and_then(|t| t.as_str()).unwrap_or("");
                    if super::filter::should_skip_title(title) {
                        continue;
                    }
                    let issue_id = issue.get("id").and_then(|id| id.as_i64()).unwrap_or(0);
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
    Ok(())
}