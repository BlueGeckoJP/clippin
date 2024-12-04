use std::{
    env::temp_dir,
    fs::{File, metadata},
    io::{BufReader, BufWriter, Read, Write},
    path::{self, Path},
};

use clap::{Parser, Subcommand, command};
use fs_extra::{
    TransitProcess, copy_items_with_progress,
    dir::{CopyOptions, TransitProcessResult},
};
use indicatif::{ProgressBar, ProgressStyle};

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
        /// Directory path to paste the file
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
    let dst_path = Path::new(path);

    let mut reader = BufReader::new(File::open(tempfile).unwrap());
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let src_path = Path::new(&content);
    if src_path.exists() {
        if src_path == path::absolute(dst_path).unwrap() {
            println!("The source and destination paths are the same");
            return;
        }

        if dst_path.join(src_path.file_name().unwrap()).exists() {
            println!("The file to be copied already exists");
            return;
        }

        let file_len = metadata(src_path).unwrap().len();

        let mut last_progress = 0;
        let progressbar = ProgressBar::new(file_len)
            .with_message(format!(
                "Copying {}",
                src_path.file_name().unwrap().to_string_lossy(),
            ))
            .with_style(
                ProgressStyle::with_template(
                    "[{percent:.cyan}%] [{elapsed_precise:.cyan}] {msg}\n{wide_bar:.cyan/blue}",
                )
                .unwrap(),
            );
        let options = CopyOptions::new();
        let handler = |process_info: TransitProcess| {
            let increased = process_info.copied_bytes - last_progress;
            progressbar.inc(increased);
            last_progress += increased;
            TransitProcessResult::ContinueOrAbort
        };
        copy_items_with_progress(&[src_path], dst_path, &options, handler).unwrap();
    } else {
        println!("The source file does not exist");
    }
}
