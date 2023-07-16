mod crypto;
mod error;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Spend {},
    Update {
        #[arg(long)]
        force: bool,
    },
    KeyPair {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Spend {}) => {
            println!("Spend");
        }
        Some(Commands::Update { force }) => {
            println!("Update. force={}", &force);
        }
        Some(Commands::KeyPair {}) => {
            crypto::generate_keypair().expect("Error generating keypair");
        }
        None => {
            println!("no command was provided");
        }
    }
}
