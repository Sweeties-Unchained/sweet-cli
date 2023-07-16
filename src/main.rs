mod crypto;
mod error;
mod init;

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
    init::init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Spend {}) => {
            println!("Spend");
        }
        Some(Commands::Update { force }) => {
            println!("Update. force={}", &force);
        }
        Some(Commands::KeyPair {}) => {
            let keypair = crypto::generate_keypair("keypair").expect("Error generating keypair");
            let keypair_retrieved =
                crypto::retrieve_keypair_from_storage("keypair").expect("Error retrieving keypair");

            use ring::signature::KeyPair;

            assert_eq!(
                keypair.public_key().as_ref(),
                keypair_retrieved.public_key().as_ref()
            )
        }
        None => {
            println!("no command was provided");
        }
    }
}
