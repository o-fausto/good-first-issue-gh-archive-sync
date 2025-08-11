use time::OffsetDateTime;

// Any archiving operations go here:
// Build the GitHub Archive URL for a given date and hour.
pub fn archive_url(year: i32, month: u32, day: u32, hour: u32) -> String {
    format!(
        "https://data.gharchive.org/{:04}-{:02}-{:02}-{:02}.json.gz",
        year, month, day, hour
    )
}

pub fn now_date() -> (i32, u32, u32, u32) {
    let now = OffsetDateTime::now_utc();
    let year = now.year();
    let month = now.month() as u32;
    let mut day = now.day() as u32;
    let hour = u32::from(now.hour());
    day = day - 1;
    (year, month, day, hour)
}

pub fn current_archive_url_and_key() -> (String, String) {
    let (year, month, day, hour) = now_date();
    let url = archive_url(year, month, day, hour);
    let archive_key = format!("{:04}-{:02}-{:02}-{:02}", year, month, day, hour);
    (url, archive_key)
}