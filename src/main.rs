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
}



fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { path } => {
            println!("Initializing at: {}", path);
        }
    }
}
