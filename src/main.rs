use clap::{arg, command, Parser};
use log::{error, info, LevelFilter};
use simple_logger::SimpleLogger;

use std::{fs, io, process::exit};

mod input;
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

    // Read file contents (or terminate if an error occurs while doing so)
    let code = fs::read_to_string(&args.filename).unwrap_or_else(|error| {
        match error.kind() {
            io::ErrorKind::NotFound => error!("File {} not found", &args.filename),
            io::ErrorKind::PermissionDenied => {
                error!("Couldn't open file due to a permission error")
            }
            _ => error!(
                "An unknown error occured while opening file {}",
                &args.filename
            ),
        };

        exit(1)
    });

    info!("Successfully opened file {}", &args.filename);

    let mut interpreter = Interpreter::new(code, args.cell_size);

    info!("Start executing program...");
    interpreter.run_to_end();
    info!("Reached end of code data. Terminating...")
}
