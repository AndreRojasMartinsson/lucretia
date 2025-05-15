use std::{fs, path::PathBuf};

use anyhow::Result;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

pub const NAME: &str = "lucretiac";

mod args;

fn main() -> Result<()> {
    let args = args::parse();
    let file_stem = args
        .input
        .file_stem()
        .expect("Expected a file stem from the input path");

    if args.output_directory.is_some() && args.output_file.is_some() {
        warn!("ignoring --out-dir flag due to -o flag");
    }

    let output_path = match (args.output_directory, args.output_file) {
        (None, Some(filename)) | (Some(_), Some(filename)) => {
            info!("Using filename {filename:?}");
            filename
        }
        (Some(dir), None) => {
            fs::create_dir_all(&dir)?;

            info!("Using directory {dir:?}");
            dir.join(file_stem)
        }
        (None, None) => {
            info!("Using filename {}", file_stem.display());
            PathBuf::from(file_stem)
        }
    };

    println!("{output_path:?}");

    Ok(())
}
