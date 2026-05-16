use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(
    name = "mo",
    about = "\n\nA work journal that takes seconds, not minutes",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
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

    /// House chores
    Home {
        message: String,
        #[command(flatten)]
        tags: Tags,
    },

    /// How are you feeling?
    Mood { message: String },

    /// Talk about feedback and/or bug reports
    Talk { message: String },

    /// Something that is fun
    Play {
        message: String,
        #[command(flatten)]
        tags: Tags,
    },

    /// Add a work entry
    Work {
        message: String,
        #[command(flatten)]
        tags: Tags,
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
pub struct Tags {
    #[arg(long)]
    blocked: bool,
    #[arg(long)]
    done: bool,
    #[arg(long)]
    feature: bool,
    #[arg(long)]
    meeting: bool,
    #[arg(long)]
    note: bool,
    #[arg(long)]
    now: bool,
    #[arg(long)]
    research: bool,
    #[arg(long)]
    todo: bool,
    #[arg(long)]
    unplanned: bool,
    #[arg(long)]
    urgent: bool,
}

impl Tags {
    pub fn to_vec(&self) -> Vec<&str> {
        let mut tags = Vec::new();
        if self.blocked {
            tags.push("blocked");
        }
        if self.done {
            tags.push("done");
        }
        if self.feature {
            tags.push("feature");
        }
        if self.meeting {
            tags.push("meeting");
        }
        if self.note {
            tags.push("note");
        }
        if self.now {
            tags.push("now");
        }
        if self.research {
            tags.push("research");
        }
        if self.todo {
            tags.push("todo");
        }
        if self.unplanned {
            tags.push("unplanned");
        }
        if self.urgent {
            tags.push("urgent");
        }
        tags.sort();

        tags
    }
}

#[derive(Subcommand)]
pub enum PeopleAction {
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

#[derive(Subcommand)]
pub enum ProjectAction {
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
