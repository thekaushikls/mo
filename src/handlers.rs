use crate::{cli, config, store};
use chrono::{Datelike, Duration, Local};
use cli::Tags;
use std::error::Error;

pub fn handle_init(path: String) -> Result<(), Box<dyn Error>> {
    config::Vault::create(path)
}

pub fn handle_login(mood: Option<String>) -> Result<(), Box<dyn Error>> {
    let vault = config::Vault::vault_path()?;

    let line = format!("{}|login", timestamp());
    store::append_line(&vault, &line)?;
    println!("Welcome back!");

    if let Some(mood) = mood {
        handle_command("mood", mood, None)?;
    }

    Ok(())
}

pub fn handle_break(message: Option<String>) -> Result<(), Box<dyn Error>> {
    let vault = config::Vault::vault_path()?;

    if let Some(message) = message {
        let line = format!("{}|break|{}", timestamp(), message);
        store::append_line(&vault, &line)?;
        return Ok(());
    }

    let line = format!("{}|break", timestamp());
    store::append_line(&vault, &line)?;
    Ok(())
}

pub fn handle_log(arg: String) -> Result<(), Box<dyn Error>> {
    let vault = config::Vault::vault_path()?;
    let lines: Vec<String>;

    if let Ok(n) = arg.parse::<usize>() {
        lines = store::read_lines(&vault, n)?;
    } else {
        let today = Local::now().date_naive();
        let arg = arg.to_lowercase();
        lines = match arg.as_str() {
            "file" => {
                println!("{}", store::get_file(&vault, today).display());
                return Ok(());
            }
            "today" => store::read_lines_by_date_range(&vault, today, today)?,
            "month" => {
                // TODO: Can move to dedicated (extension) Trait + Impl
                let start_date = today.with_day(1).unwrap();
                // println!("Start: {}", start_date);
                let end_date = start_date + chrono::Months::new(1) - Duration::days(1); // +1 Month -1 Day to get last day of the month
                store::read_lines_by_date_range(&vault, start_date, end_date)?
            }
            _ => {
                return Err(format!("Invalid argument: {}", arg).into());
            }
        };
    }

    if lines.is_empty() {
        println!("No entries found!");
    } else {
        for line in &lines {
            println!("{}", line);
        }
    }

    // let lines = if arg.to_lowercase() == "today" {
    //     let date_str = Local::now().format("%Y-%m-%d").to_string();
    //     let all = store::read_lines(&vault, usize::MAX)?;
    //     all.into_iter()
    //         .filter(|l| l.starts_with(&date_str))
    //         .collect()
    // } else {
    //     let count: usize = arg.parse().unwrap_or(5);
    //     store::read_lines(&vault, count)?
    // };

    // if lines.is_empty() {
    //     println!("No entries this week.");
    //     return Ok(());
    // }

    // for line in &lines {
    //     println!("{}", line);
    // }

    Ok(())
}

pub fn handle_command(
    category: &str,
    message: String,
    tags: Option<Tags>,
) -> Result<(), Box<dyn Error>> {
    let vault = config::Vault::vault_path()?;

    let mut line: String = format!("{}|{}|{}", timestamp(), category, message);

    if let Some(tags) = tags {
        let tag_list = tags.to_vec();
        if !tag_list.is_empty() {
            line.push_str(&format!("|tags={}", tag_list.join(",")));
        }
    }

    store::append_line(&vault, &line)?;
    Ok(())
}

// TODO: Move a separate struct.
pub fn handle_today() -> Result<(), Box<dyn Error>> {
    let vault = config::Vault::vault_path()?;

    let date_str = Local::now().format("%Y-%m-%d").to_string();
    let all = store::read_lines(&vault, usize::MAX)?;
    let _lines: Vec<String> = all
        .into_iter()
        .filter(|l| l.starts_with(&date_str))
        .collect();

    // 0 -> timestamp
    // 1 -> type
    // 2 -> comment | optional
    // 3 -> flags (ignoring for now) | optional

    const LINE_LENGTH: usize = 55; //TODO: Move to global config. This should be user-configurable.
    println!();
    for line in &_lines {
        let parts: Vec<&str> = line.split('|').collect();
        let time_str = &parts[0][11..16]; //TODO: Fix unsafe slicing
        let type_str = parts[1];

        let comment_str = if parts.len() > 2 {
            if parts[2].len() > LINE_LENGTH {
                // TODO: Refactor, parts[2].len() being called multiple times.
                let _slice = parts[2].len().min(LINE_LENGTH);
                // TODO: ({} more) should be changed to count words instead of characters.
                format!(
                    ": {}... ({} more)",
                    &parts[2][.._slice],
                    parts[2].len() - LINE_LENGTH
                )
            } else {
                // TODO: Remove the ':', needs to be placed conditionally - if comments exist.
                format!(": {}", parts[2])
            }
        } else {
            String::new()
        };

        // TODO: Remove hardcoded value '8' for fixed width. Either all "type" strings should be
        // 4 letters (prefereable), or should be calculated dynamically, before the print-loop.
        println!("{time_str:} {type_str:<8}{comment_str}");
    }
    println!();

    Ok(())
}

pub fn handle_logout() -> Result<(), Box<dyn Error>> {
    let vault = config::Vault::vault_path()?;

    let line = format!("{}|logout", timestamp());
    store::append_line(&vault, &line)?;
    println!("Goodbye!");
    Ok(())
}

fn timestamp() -> String {
    Local::now().format("%Y-%m-%dT%H:%M:%S%.9f%:z").to_string()
}
