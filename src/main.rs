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
    /// Specify files to copy
    #[command(alias = "c")]
    Copy {
        /// File path to be copied
        path: String,
    },
    /// Paste a file specified by the Copy command
    #[command(alias = "p")]
    Paste {
        /// Directory path or file path to paste the file
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
    let mut dst_path = Path::new(path);

    let mut reader = BufReader::new(File::open(tempfile).unwrap());
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let src_path = Path::new(&content);
    if src_path.exists() {
        let joined_path = dst_path.join(src_path.file_name().unwrap());
        if dst_path.is_dir() {
            dst_path = &joined_path;
        }

        fs::copy(src_path, dst_path).unwrap();
    } else {
        println!("The source file does not exist");
    }
}
