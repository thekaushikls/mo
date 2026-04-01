mod registry;
use std::{error::Error, fs, path::Path};
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
    Start,

    /// End work day
    End,

    /// Add an entity (Project) to the registry
    Add {
        #[command(subcommand)]
        entity: AddEntity,
    }
}


#[derive(Subcommand)]
enum AddEntity {
    /// Add a project
    Project {
        /// Project name
        name: String,
        /// Alias (repeatable)
        alias: Vec<String>,
    },
    
    /// Add a person
    Person {
        /// Person name
        name: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Command::Init { path } => handle_init(path)?,
        Command::Start => handle_start(),
        Command::End => handle_end(),

        Command::Add {entity} => match entity {
            AddEntity::Project{name, alias} => handle_add_entity(name)?,
            AddEntity::Person { name } => handle_add_entity(name)?,
        }
    }

    Ok(())
}

fn handle_init(path: String) -> Result<(), Box<dyn Error>> {
    let registry_path = Path::new(&path).join("registry.toml");

    if registry_path.exists() {
        println!("Registry already exists at: {}", registry_path.display());
    } else {
        fs::create_dir_all(&path)?;
        let registry = registry::Registry::default();
        registry.save(&registry_path)?;
        println!("Created registry at: {}", registry_path.display());
    }

    Ok(())
}

fn handle_start() {
    println!("Welcome back!");
}

fn handle_end() {
    println!("Goodbye!");
}

fn handle_add_entity(name: String) -> Result<(), Box<dyn Error>> {
    println!("Added {} successfully!", name);
    
    Ok(())
}