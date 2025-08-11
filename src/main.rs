use rusqlite::{Connection};
use std::env;
use ureq;

mod db;
mod archive;
mod filter;
mod fetch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get DB path from command line argument or use default
    let db_path = env::args().nth(1).unwrap_or_else(|| "good_first_issues.db".to_string());

     // Validate the database path
    if let Err(e) = db::validate_db_path(&db_path) {
        eprintln!("{}", e);
        return Ok(());
    }


    // Try to open the database safely
    let conn = match Connection::open(&db_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error opening database '{}': {}", db_path, e);
            return Ok(());
        }
    };
    db::initialize_db(&conn)?;


    // Step 2: Fetch the list of repositories to monitor
    let _repositories: Vec<String> = conn
        .prepare("SELECT full_name FROM repositories")?
        .query_map([], |row| row.get(0))?
        .collect::<Result<_, _>>()?;

    let (url, archive_key) = archive::current_archive_url_and_key();

    if db::is_archive_processed(&conn, &archive_key)? {
        println!(
            "Archive {}.json.gz already processed. Printing issues from DB:\n",
            archive_key
        );
        db::print_issues_from_db(&conn)?;
        return Ok(());
    }

    println!("Downloading file from: {}", url);

    // Use ureq for HTTP requests
    let response = ureq::get(&url).call();

    if response.is_err() {
        eprintln!("Error fetching data from URL: {}", url);
        return Ok(());
    }
    let _response = response.unwrap();

    println!("Decompressing and filtering issues...");
    let buffered = fetch::download_and_decompress(&url)?;
    filter::process_issues_from_reader(&conn, buffered)?;

    // After processing and inserting issues, print out all issues in the database
    println!("\nList of 'good first issues' in the database:");
    db::print_issues_from_db(&conn)?;
    // Mark the archive as processed
    db::mark_archive_processed(&conn, &archive_key)?;

    Ok(())
}


