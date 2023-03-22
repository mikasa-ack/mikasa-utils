use clap::{Parser, Subcommand};
use mikasa_utils::command;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    InstantiateGrid {
        #[arg(short, long)]
        dimension: Option<u8>,
        /// A boolean to indicate if the command must be executed or not.
        #[arg(short, long)]
        execute: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::InstantiateGrid { dimension, execute }) => {
            command::instantiate_grid(dimension.unwrap_or(10), execute);
        }
        None => {}
    }
}
