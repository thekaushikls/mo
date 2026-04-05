use std::{error::Error, fs, path::Path, path::PathBuf};
use chrono::{Local, Datelike, Duration, NaiveDate};


/// Get Monday from current ISO week
fn week_monday(date: NaiveDate) -> NaiveDate {
    date - Duration::days(date.weekday().num_days_from_monday() as i64)
}

/// Generate weekly markdown file
// pub fn create_weekly_file(date: NaiveDate) -> String {
//     let monday = week_monday(date);
//     let week_label = monday.format("%G-W%V").to_string();

//     // Create template header manually
//     let mut content = format!("# {}\n\n", week_label);
//     content.push_str("## WeeklySummary\n");
//     content.push_str("<!-- Retro: decisions, contributions, blockers, etc. -->\n\n\n");
//     content.push_str("---\n");

//     let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
//     for (i, day_name) in days.iter().enumerate() {
//         let day_date = monday + Duration::days(i as i64);
//         let date_str = day_date.format("%Y-%m-%d").to_string();

//         content.push_str(&format!("\n## {} {}\n", day_name, date_str));
//         content.push_str("login: | logout: | mood:\n\n");
//         content.push_str("-\n");
//     }
//     content
// }

/// Get current week file
// pub fn weekly_file_path(vault: &Path, date: NaiveDate) -> PathBuf {
//     let monday = week_monday(date);
//     let week_label = monday.format("%G-W%V").to_string();    
//     vault.join("weekly").join(format!("{}.md", week_label))
// }

/// Get log file path
pub fn log_file_path(vault: &Path, date: NaiveDate) -> PathBuf {
    let monday = week_monday(date);
    let week_label = monday.format("%G-W%V").to_string();    
    vault.join("logs").join(format!("{}.log", week_label))
}

/// Ensure weekly file exists
// pub fn ensure_weekly_file(vault: &Path, date: NaiveDate) -> Result<PathBuf, Box<dyn Error>> {
//     let path = weekly_file_path(vault, date);
//     if !path.exists() {
//         fs::create_dir_all(path.parent().unwrap())?;
//         let content = create_weekly_file(date);
//         fs::write(&path, &content)?;
//     }
//     Ok(path)
// }

/// Append to log file
pub fn append_log(vault: &Path, line: &str) -> Result<(), Box<dyn Error>> {

    let today = Local::now().date_naive();
    let path = log_file_path(vault, today);
    fs::create_dir_all(path.parent().unwrap())?;
    
    use std::fs::OpenOptions;
    use std::io::Write;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    writeln!(file, "{}", line)?;
    Ok(())
}
