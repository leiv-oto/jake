mod creation;
use creation::{creation, Lang};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jake")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    New {
        /// Project name
        name: String,

        /// Target language
        #[arg(short, long, value_enum, default_value_t = Lang::Kotlin)]
        lang: Lang,

        /// Set up a mixed Java/Kotlin structure
        #[arg(short, long)]
        mixed: bool,
    },
    /// Run the current project
    Run {},
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name, lang, mixed } => {
            creation(&name, lang, mixed);
        }
        Commands::Run {} => {
            println!("Lol, this thing still isn't ready. GYU 🦭");
        }
    }
}