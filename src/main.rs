mod registry;
mod weekly;

use std::{error::Error, fs, path::Path};
use chrono::{Local};
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name="mo", about="CLI tool to log work")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize a new registry
    Init {
        /// Path for the registry directory
        #[arg(long, default_value="./sample/vault")]
        path: String,
    },

    /// Start work day
    Login,

    /// End work day
    Logout,

    /// Add an entity (Project) to the registry
    Add {
        #[command(subcommand)]
        entity: AddEntity,
    },

    /// Add a work entry
    Work {
        message: String,
        #[command(flatten)]
        flags: WorkFlags,
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
    todo: bool,
    #[arg(long)]
    unplanned: bool,
    #[arg(long)]
    urgent: bool,
}

impl WorkFlags {
    fn to_vec(&self) -> Vec<&str> {
        let mut flags = Vec::new();
        if self.blocked { flags.push("blocked"); }
        if self.done { flags.push("done"); }
        if self.feature { flags.push("feature"); }
        if self.meeting { flags.push("meeting"); }
        if self.todo { flags.push("todo"); }
        if self.unplanned { flags.push("unplanned"); }
        if self.urgent { flags.push("urgent"); }

        flags
    }
}

#[derive(Subcommand)]
enum AddEntity {
    /// Add a project
    Project {
        /// Project name
        name: String,
        /// Alias (repeatable)
        #[arg(long)]
        alias: Vec<String>,
    },
    
    /// Add a person
    Person {
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
        Command::Init { path } => handle_init(path)?,
        Command::Login => handle_login()?,
        Command::Logout => handle_logout()?,
        Command::Work { message, flags } => handle_work(message, flags)?,
        
        Command::Add {entity} => match entity {
            AddEntity::Project{name, alias} => handle_add_project(name, alias)?,
            AddEntity::Person {name, alias} => handle_add_person(name, alias)?,
        }
    }

    Ok(())
}

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

fn handle_login() -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);
    let now = Local::now();
    
    let line = format!("{}|login", now.to_rfc3339());
    weekly::append_log(vault, &line)?;
    println!("Welcome back!");
    Ok(())
    
}

fn handle_logout() -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let vault = Path::new(&registry.vault.path);
    let now = Local::now();
    
    let line = format!("{}|logout", now.to_rfc3339());
    weekly::append_log(vault, &line)?;
    println!("Goodbye!");
    Ok(())
}

fn handle_work(message: String, flags: WorkFlags) -> Result<(), Box<dyn Error>> {
    let registry = registry::Registry::load()?;
    let now = Local::now();
    let vault = Path::new(&registry.vault.path);

    let mut line = format!("{}|work|{}", now.to_rfc3339(), message);
    let flag_list = flags.to_vec();
    if !flag_list.is_empty() {
        line.push_str(&format!("|flags={}", flag_list.join(",")));
    }

    weekly::append_log(vault, &line)?;
    Ok(())
}

fn handle_add_project(name: String, aliases: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut registry = registry::Registry::load()?;

    if registry.find_project(&name).is_some() {
        
        println!("Project {} already exists.", name);
        return Ok(())
    }

    let new_project = registry::Project {
        name: name.clone(),
        aliases,
        status: "active".into(),
    };
    registry.projects.push(new_project);
    registry.save()?;
    
    println!("Added project `{}`.", name);
    Ok(())
}

fn handle_add_person(name: String, aliases: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut registry = registry::Registry::load()?;

    if registry.find_person(&name).is_some() {
        println!("Person {} already exists.", name);
        return Ok(())
    }

    let new_person = registry::Person {
        name: name.clone(),
        aliases,
    };
    registry.people.push(new_person);
    registry.save()?;

    println!("Added person `{}`.", name);
    Ok(())
}
