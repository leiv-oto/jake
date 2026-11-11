mod creation;
use creation::creation;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "jake")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Lang {
    Kotlin,
    Java,
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
    },
    /// Run the current project
    Run {},
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New { name, lang } => {
            creation(name, lang.to_possible_value().unwrap().get_name());
        }
        Commands::Run {} => {
            println!("Lol, this thing still isn't ready. GYU 🦭");
        }
    }
}