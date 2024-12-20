use std::{
    env::temp_dir,
    fs::{File, metadata},
    io::{BufReader, BufWriter, Read, Write},
    path::{self, Path},
};

use clap::{Parser, Subcommand, command};
use env_logger::Env;
use fs_extra::{
    TransitProcess, copy_items_with_progress,
    dir::{CopyOptions, TransitProcessResult},
};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info};

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
        /// File paths to be copied
        path: Vec<String>,
    },
    /// Paste files specified by the Copy command
    #[command(alias = "p")]
    Paste {
        /// Directory path to paste the files
        #[clap(default_value_t = String::from("./"))]
        path: String,
    },
}

fn main() {
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let args = Args::parse();

    match &args.command {
        Commands::Copy { path } => copy_fn(path),
        Commands::Paste { path } => paste_fn(path),
    }
}

fn copy_fn(paths: &[String]) {
    let tempfile = temp_dir().join("clippin.txt");
    let absolute_paths = paths
        .iter()
        .filter_map(|p| {
            let absolute_path = path::absolute(p).unwrap();
            if absolute_path.exists() {
                Some(absolute_path.to_string_lossy().to_string())
            } else {
                error!("{} was ignored because it does not exist", p);
                None
            }
        })
        .collect::<Vec<String>>();

    let paths_string = absolute_paths.join("\n");

    let mut writer = BufWriter::new(File::create(&tempfile).unwrap());
    writer.write_all(paths_string.as_bytes()).unwrap();
    info!(
        "The following files have been set as the source files \n{}",
        paths.join("\n")
    );
}

fn paste_fn(path: &String) {
    let tempfile = temp_dir().join("clippin.txt");
    let dst_path = Path::new(path);

    let mut reader = BufReader::new(File::open(tempfile).unwrap());
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let src_paths = content
        .split("\n")
        .filter_map(|p_str| {
            let p = Path::new(p_str);
            if p == path::absolute(dst_path).unwrap() {
                error!(
                    "Ignore {} because the source and destination paths are the same",
                    p_str
                );
                return None;
            }

            if dst_path.join(p.file_name().unwrap()).exists() {
                error!(
                    "Ignore {} because the destination file already exists",
                    p_str
                );
                return None;
            }

            if !p.exists() {
                error!("Ignore {} because it does not exist", p_str);
                return None;
            }

            Some(p)
        })
        .collect::<Vec<&Path>>();

    for src_path in &src_paths {
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
        progressbar.finish();
    }

    info!("{} files have been copied", src_paths.len());
}
