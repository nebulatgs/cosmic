use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod commands;
use commands::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cosmic {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        path: PathBuf,
        #[clap(short, long)]
        c: bool,
    },
    New {
        name: String,
        #[clap(short, long)]
        c: bool,
    },
}

fn main() {
    let cli = Cosmic::parse();

    let res = match &cli.command {
        Commands::Init { path, c } => init::init(path, *c),
        Commands::New { name, c } => new::new(name, *c),
    };

    match res {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
