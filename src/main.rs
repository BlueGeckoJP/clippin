use clap::{Parser, Subcommand, command};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Copy { path: String },
    Paste { path: String },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Copy { path } => copy_fn(path),
        Commands::Paste { path } => paste_fn(path),
    }
}

fn copy_fn(path: &String) {}

fn paste_fn(path: &String) {}
