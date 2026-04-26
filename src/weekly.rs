use chrono::{Datelike, Duration, Local, NaiveDate};
use std::{error::Error, fs, path::Path, path::PathBuf};

/// Get Monday from current ISO week
fn week_monday(date: NaiveDate) -> NaiveDate {
    date - Duration::days(date.weekday().num_days_from_monday() as i64)
}

/// Get log file path
pub fn log_file_path(vault: &Path, date: NaiveDate) -> PathBuf {
    let monday = week_monday(date);
    let week_label = monday.format("%G-W%V").to_string();
    vault.join("logs").join(format!("{}.log", week_label))
}

/// Append to log file
pub fn append_log(vault: &Path, line: &str) -> Result<(), Box<dyn Error>> {
    let today = Local::now().date_naive();
    let path = log_file_path(vault, today);
    fs::create_dir_all(path.parent().unwrap())?;

    use std::fs::OpenOptions;
    use std::io::Write;
    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;

    writeln!(file, "{}", line)?;
    Ok(())
}

/// Read from log file
pub fn read_lines(vault: &Path, count: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let today = Local::now().date_naive();
    let path = log_file_path(vault, today);

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
