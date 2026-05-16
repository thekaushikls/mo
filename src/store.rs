use chrono::{Datelike, Duration, Local, NaiveDate};
use std::{error::Error, fs, fs::OpenOptions, io::Write, path::Path, path::PathBuf};

/// Get Monday from current ISO week
fn week_monday(date: NaiveDate) -> NaiveDate {
    date - Duration::days(date.weekday().num_days_from_monday() as i64)
}

/// Get log filepath by date
pub fn get_file(vault: &Path, date: NaiveDate) -> PathBuf {
    let monday = week_monday(date);
    let week_label = monday.format("%G-W%V").to_string();
    vault.join("logs").join(format!("{}.log", week_label))
}

/// Get all log filepaths by date range
pub fn get_files(vault: &Path, start_date: NaiveDate, end_date: NaiveDate) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut week_start = week_monday(start_date);

    while week_start <= end_date {
        let path = get_file(vault, week_start);
        if path.exists() {
            files.push(path);
        }
        week_start += Duration::days(7);
    }

    return files;
}

/// Append to log file
pub fn append_line(vault: &Path, line: &str) -> Result<(), Box<dyn Error>> {
    let today = Local::now().date_naive();
    let path = get_file(vault, today);

    fs::create_dir_all(path.parent().unwrap())?;
    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;

    writeln!(file, "{}", line)?;
    Ok(())
}

/// Read 'n' lines from the end of file
pub fn read_lines(vault: &Path, count: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let today = Local::now().date_naive();
    let path = get_file(vault, today);

    if !path.exists() {
        return Ok(vec![]);
    }

    let contents = fs::read_to_string(&path);
    let lines: Vec<String> = contents?
        .lines()
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect();
    let start = if lines.len() > count {
        lines.len() - count
    } else {
        0
    };
    Ok(lines[start..].to_vec())
}

// Read contents based on date range
pub fn read_lines_by_date_range(vault: &Path, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<String>, Box<dyn Error>> {
    let paths = get_files(&vault, start_date, end_date);
    let mut log_lines: Vec<String> = Vec::new();

    for path in paths {
        if path.exists() {
            let contents = fs::read_to_string(path);
            let lines: Vec<String> = contents?
                .lines()
                .filter(|line| !line.is_empty())
                .map(String::from)
                .collect();
            log_lines.extend_from_slice(&lines);
        }
    }

    // TODO: Perform checks in single loop
    let start_timestamp = start_date.format("%Y-%m-%d").to_string();
    let end_timestamp = end_date.format("%Y-%m-%d").to_string();
    log_lines = log_lines
        .into_iter()
        .filter(|line| {
            if let Some(date) = line.get(..10) {
                date >= start_timestamp.as_str() && date <= end_timestamp.as_str()
            } else {
                false
            }
        })
        .collect();

    Ok(log_lines.to_vec())
}
