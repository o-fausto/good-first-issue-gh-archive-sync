## What is this?

This is a fork of https://github.com/ArturLinnik/good-first-issue-gh-archive-sync.
I have simplified it to use classes and a little better error handling.


This is a script made in `rust` (just because I wanted, it's an overkill) that adds or updates the status of the issues with the tag `good first issue` in a local SQLite database from a table `repositories` that contains the repositories you want to watch for changes in the events of GH Archive.

Why? Because I need it for a project in which a user will be notified if a starred repository has a new issue with the tag `good first issue`.

## Get started

1. Install SQLite and or SQLite development library for Windows
2. Add an SQLite database named `good_first_issues.db`.
3. Add the full name of the repositories you want to watch to the table `repositories` ('owner/repo').
4. Run the script with `cargo run`.
5. Prints a list of the issues retrieved.
6. Compile the script and run it every hour by adding it to a cronjob

