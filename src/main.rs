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
    /// Crea un nuevo proyecto especificando el lenguaje y el nombre
    New {
        #[arg(value_enum)]
        ttype: Lang,

        /// Nombre del proyecto
        name: String,
    },
    /// Ejecuta el proyecto actual
    Run {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { ttype, name } => {
            println!("Creando {} en {:?}", name, ttype);
        }
        Commands::Run {} => {
            println!("Ejecutando proyecto...");
        }
    }
}