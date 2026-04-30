mod entity;
mod registry;
mod weekly;

use chrono::Local;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use std::{error::Error, fs, io::stdout, path::Path};

#[derive(Parser)]
#[command(name = "mo", about = "CLI tool to log work", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate shell completions
    #[command(hide = true)]
    Completions {
        /// Shell type (bash, zsh, fish)
        shell: Shell,
    },

    /// Initialize a new registry
    Init {
        /// Path for the registry directory
        #[arg(long, default_value = ".")]
        path: String,
    },

    /// Start work day
    Login {
        #[arg(long)]
        mood: Option<String>,
    },

    /// Start / Stop a break
    Break { message: Option<String> },

    /// How are you feeling?
    Mood { mood: String },

    /// Talk about feedback and/or bug reports
    Talk { message: String },

    /// Jot down a note
    Note { message: String },

    /// Add a work entry
    Work {
        message: String,
        #[command(flatten)]
        flags: WorkFlags,
    },

    /// Show recent entries (default: 5)
    Log {
        #[arg(default_value = "5")]
        arg: String,
    },

    /// Show entries from current day
    Today,

    /// End work day
    Logout,

    /// Manage Projects
    Project {
        #[command(subcommand)]
        action: ProjectAction,
    },

    /// Manage People
    People {
        #[command(subcommand)]
        action: PeopleAction,
    },
}

#[derive(clap::Args)]
struct WorkFlags {
    #[arg(long)]
    blocked: bool,
    #[arg(long)]
    done: bool,
    #[arg(long)]
    feature: bool,
    #[arg(long)]
    meeting: bool,
    #[arg(long)]
    now: bool,
    #[arg(long)]
    todo: bool,
    #[arg(long)]
    unplanned: bool,
    #[arg(long)]
    urgent: bool,
}

impl WorkFlags {
    fn to_vec(&self) -> Vec<&str> {
        let mut flags = Vec::new();
        if self.blocked {
            flags.push("blocked");
        }
        if self.done {
            flags.push("done");
        }
        if self.feature {
            flags.push("feature");
        }
        if self.meeting {
            flags.push("meeting");
        }
        if self.now {
            flags.push("now");
        }
        if self.todo {
            flags.push("todo");
        }
        if self.unplanned {
            flags.push("unplanned");
        }
        if self.urgent {
            flags.push("urgent");
        }

        flags
    }
}

#[derive(Subcommand)]
enum ProjectAction {
    /// List all projects
    Ls,

    /// Add a project
    Add {
        /// Projects name
        name: String,

        /// Alias (repeatable)
        #[arg(long)]
        alias: Vec<String>,
    },
}

#[derive(Subcommand)]
enum PeopleAction {
    /// List all people
    Ls,

    /// Add a person
    Add {
        /// Person name
        name: String,

        /// Alias (repeatable)
        #[arg(long)]
        alias: Vec<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Completions { shell } => {
            generate(shell, &mut Cli::command(), "mo", &mut stdout());
        }
        Command::Init { path } => handle_init(path)?,
        Command::Login { mood } => handle_login(mood)?,
        Command::Mood { mood } => handle_mood(mood)?,
        Command::Break { message } => handle_break(message)?,
        Command::Note { message } => handle_note(message)?,
        Command::Work { message, flags } => handle_work(message, flags)?,
        Command::Log { arg } => handle_log(arg)?,
        Command::Logout => handle_logout()?,
        Command::Talk { message } => handle_talk(message)?,

        Command::Today => handle_today()?,

        // Manage Projects
        Command::Project { action } => match action {
            ProjectAction::Ls => entity::Project::list()?,
            ProjectAction::Add { name, alias } => entity::Project::add(name, alias)?,
        },

        // Manage People
        Command::People { action } => match action {
            PeopleAction::Ls => entity::Person::list()?,
            PeopleAction::Add { name, alias } => entity::Person::add(name, alias)?,
        },
    }

    Ok(())
}

// All Handlers

fn handle_init(path: String) -> Result<(), Box<dyn Error>> {
    let registry_path = Path::new("mo.toml");

    if registry_path.exists() {
        println!("mo.toml already exists at");
    } else {
        fs::create_dir_all(&path)?;
        let registry = registry::Registry {
            vault: registry::VaultConfig { path },
            ..Default::default()
        };

        registry.save()?;
        println!("Created registry at: {}", registry_path.display());
    }

    Ok(())
}

fn handle_login(mood: Option<String>) -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);
    let now = Local::now();

    let line = format!("{}|login", now.format("%Y-%m-%dT%H:%M:%S%.9f%:z"));
    weekly::append_log(vault, &line)?;
    println!("Welcome back!");

    if let Some(mood) = mood {
        handle_mood(mood)?;
    }

    Ok(())
}

fn handle_mood(mood: String) -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);
    let now = Local::now();

    let line = format!(
        "{}|mood|{}",
        now.format("%Y-%m-%dT%H:%M:%S%.9f%:z"),
        mood
    );
    weekly::append_log(vault, &line)?;
    Ok(())
}

fn handle_break(message: Option<String>) -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);
    let now = Local::now();

    if let Some(message) = message {
        let line = format!(
            "{}|break|{}",
            now.format("%Y-%m-%dT%H:%M:%S%.9f%:z"),
            message
        );
        weekly::append_log(vault, &line)?;
        return Ok(());
    }

    let line = format!("{}|break", now.format("%Y-%m-%dT%H:%M:%S%.9f%:z"));
    weekly::append_log(vault, &line)?;
    Ok(())
}

fn handle_work(message: String, flags: WorkFlags) -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);
    let now = Local::now();

    let mut line = format!(
        "{}|work|{}",
        now.format("%Y-%m-%dT%H:%M:%S%.9f%:z"),
        message
    );
    let flag_list = flags.to_vec();
    if !flag_list.is_empty() {
        line.push_str(&format!("|flags={}", flag_list.join(",")));
    }

    weekly::append_log(vault, &line)?;
    Ok(())
}

fn handle_note(message: String) -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);
    let now = Local::now();

    let line = format!(
        "{}|note|{}",
        now.format("%Y-%m-%dT%H:%M:%S%.9f%:z"),
        message
    );
    weekly::append_log(vault, &line)?;
    Ok(())
}

fn handle_talk(message: String) -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);
    let now = Local::now();

    let line = format!(
        "{}|talk|{}",
        now.format("%Y-%m-%dT%H:%M:%S%.9f%:z"),
        message
    );
    weekly::append_log(vault, &line)?;
    Ok(())
}

fn handle_log(arg: String) -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);

    if arg.to_lowercase() == "file" {
        let today = Local::now().date_naive();
        println!("{}", weekly::log_file_path(vault, today).display());
        return Ok(());
    }

    let lines = if arg.to_lowercase() == "today" {
        let date_str = Local::now().format("%Y-%m-%d").to_string();
        let all = weekly::read_lines(vault, usize::MAX)?;
        all.into_iter()
            .filter(|l| l.starts_with(&date_str))
            .collect()
    } else {
        let count: usize = arg.parse().unwrap_or(5);
        weekly::read_lines(vault, count)?
    };

    if lines.is_empty() {
        println!("No entries this week.");
        return Ok(());
    }

    for line in &lines {
        println!("{}", line);
    }

    Ok(())
}

// TODO: Move a separate struct.
fn handle_today() -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);

    let date_str = Local::now().format("%Y-%m-%d").to_string();
    let all = weekly::read_lines(vault, usize::MAX)?;
    let _lines: Vec<String> = all.into_iter()
        .filter(|l| l.starts_with(&date_str))
        .collect();

    // 0 -> timestamp
    // 1 -> type
    // 2 -> comment | optional
    // 3 -> flags (ignoring for now) | optional

    const LINE_LENGTH: usize = 55; //TODO: Move to global config. This should be user-configurable.
    println!("");
    for line in &_lines {
        let parts: Vec<&str> = line.split('|').collect();
        let time_str = &parts[0][11..16];
        let type_str = parts[1];
        
        let comment_str = if parts.len() > 2 {
            if parts[2].len() > LINE_LENGTH {
                // TODO: Refactor, parts[2].len() being called multiple times.
                let _slice = parts[2].len().min(LINE_LENGTH);
                // TODO: ({} more) should be changed to count words instead of characters.
                format!(": {}... ({} more)", &parts[2][.._slice], parts[2].len() - LINE_LENGTH)
            } else {
                // TODO: Remove the ':', needs to be placed conditionally - if comments exist.
                format!(": {}", parts[2].to_string())
            }
        } else {
            String::new()
        };

        // TODO: Remove hardcoded value '8' for fixed width. Either all "type" strings should be 
        // 4 letters (prefereable), or should be calculated dynamically, before the print-loop.
        println!("{time_str:} {type_str:<8}{comment_str}");
    }
    println!("");

    Ok(())
}

fn handle_logout() -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);
    let now = Local::now();

    let line = format!("{}|logout", now.format("%Y-%m-%dT%H:%M:%S%.9f%:z"));
    weekly::append_log(vault, &line)?;
    println!("Goodbye!");
    Ok(())
}
