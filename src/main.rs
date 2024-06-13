use clap::{arg, command, Parser};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

use std::process::exit;

mod interpreter;
mod modular;
use interpreter::*;

#[derive(Parser)]
#[command(
    version,
    about = "A Brainf**k interpreter written in Rust with minimal dependencies"
)]
struct Args {
    /// Brainf**k file to execute
    #[arg(default_value = DEFAULT_FILENAME)]
    filename: String,

    /// The memory size in bytes/cells to allocate for the program
    #[arg(short = 'm', long = "mem", default_value_t = DEFAULT_CELL_SIZE, value_name = "memory")]
    cell_size: usize,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    // Obtain command line parameters
    let args = Args::parse();

    // Initialize logger
    let log_level = if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Warn
    };
    SimpleLogger::new().with_level(log_level).init().unwrap();

    let mut interpreter =
        Interpreter::new_from_path(&args.filename, args.cell_size).unwrap_or_else(|_| exit(1));

    info!("Start executing program...");
    interpreter.run_to_end();
    info!("Reached end of code data. Terminating...")
}
