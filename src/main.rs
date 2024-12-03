use std::{
    env::temp_dir,
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{self, Path},
};

use clap::{Parser, Subcommand, command};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(alias = "c")]
    Copy { path: String },
    #[command(alias = "p")]
    Paste {
        #[clap(default_value_t = String::from("./"))]
        path: String,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Copy { path } => copy_fn(path),
        Commands::Paste { path } => paste_fn(path),
    }
}

fn copy_fn(path: &String) {
    let tempfile = temp_dir().join("clippin.txt");
    let p = path::absolute(path).unwrap();

    if p.exists() {
        let mut writer = BufWriter::new(File::create(tempfile).unwrap());
        writer
            .write_all(p.display().to_string().as_bytes())
            .unwrap();
    } else {
        println!("The specified file does not exist");
    }
}

fn paste_fn(path: &String) {
    let tempfile = temp_dir().join("clippin.txt");
    let mut p = Path::new(path);

    let mut reader = BufReader::new(File::open(tempfile).unwrap());
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let dst_path = Path::new(&content);
    if dst_path.exists() {
        let joined_path = p.join(dst_path.file_name().unwrap());
        if p.is_dir() {
            p = &joined_path;
        }

        fs::copy(dst_path, p).unwrap();
    } else {
        println!("The source file does not exist");
    }
}
