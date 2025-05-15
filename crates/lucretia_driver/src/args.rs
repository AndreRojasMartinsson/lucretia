use shadow_rs::shadow;
use std::{env, path::PathBuf, process::exit};

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

// Ignore the error below
shadow!(build);

#[derive(Parser, Debug)]
pub struct Arguments {
    pub input: PathBuf,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// Optimization level to use when compiling
    #[arg(short = 'O', long = "opt-level", value_name = "LEVEL", num_args(0..=1), default_value="0")]
    pub optimization: u8,

    /// Links to shared library or shared object
    #[arg(short = 'l', value_name = "LIBRARY", num_args = 1..)]
    pub libraries: Vec<String>,

    /// Add search path to shared libraries, directory containing *.so, *.dll or *.dlyb files.
    #[arg(short = 'L', value_name = "LIBRARY_DIR", num_args= 1..)]
    pub library_directories: Vec<PathBuf>,

    /// Write output to <filename>
    #[arg(short = 'o', value_name = "FILENAME")]
    pub output_file: Option<PathBuf>,

    /// Write compiler artifacts to compiler-chosen filename in <dir>
    #[arg(long = "out-dir", value_name = "DIR")]
    pub output_directory: Option<PathBuf>,

    /// Emit debug information to binary
    #[arg(short = 'g', default_value_t = false)]
    pub debug: bool,

    /// Print version of compiler to stdout
    #[arg(long = "version", default_value_t = false)]
    show_version: bool,
}

fn get_build_date() -> String {
    let build_time: &str = build::BUILD_TIME;
    let segments: Vec<_> = build_time.split(" ").collect();

    segments[0].to_owned()
}

fn show_version() -> ! {
    println!(
        "{name} {version} ({hash} {built})\n{authors}",
        name = super::NAME,
        version = crate_version!(),
        hash = build::SHORT_COMMIT,
        built = get_build_date(),
        authors = crate_authors!()
    );

    exit(0)
}

pub fn parse() -> Arguments {
    if env::args().any(|x| &x == "--version") {
        show_version()
    }

    let args = Arguments::parse();

    // Initialize the logger based on the verbosity flag
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Parsed arguments");

    args
}
