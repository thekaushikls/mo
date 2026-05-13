mod cli;
mod entity;
mod handlers;
mod registry;
mod weekly;

use std::{error::Error, io::stdout};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::{Cli, Command, PeopleAction, ProjectAction};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        // Meta Config
        Command::Completions { shell } => {
            generate(shell, &mut Cli::command(), "mo", &mut stdout());
        }
        Command::Today => handlers::handle_today()?,
        Command::Log { arg } => handlers::handle_log(arg)?,
        // Basic usage
        Command::Init { path } => handlers::handle_init(path)?,
        Command::Login { mood } => handlers::handle_login(mood)?,
        Command::Logout => handlers::handle_logout()?,
        Command::Break { message } => handlers::handle_break(message)?,

        // supports Tags
        Command::Home { message, tags } => handlers::handle_command("home", message, Some(tags))?,
        Command::Play { message, tags } => handlers::handle_command("play", message, Some(tags))?,
        Command::Work { message, tags } => handlers::handle_command("work", message, Some(tags))?,

        // does not support Tags
        Command::Mood { message } => handlers::handle_command("mood", message, None)?,
        Command::Talk { message } => handlers::handle_command("talk", message, None)?,

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
