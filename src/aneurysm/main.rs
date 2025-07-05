use clap::{arg, command, Parser};
use flexi_logger::Logger;
use log::LevelFilter;

use std::process::exit;

use aneurysm::*;
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

    /// Whether or not to echo characters written to stdin
    #[arg(short, long)]
    echo: bool,
}

fn main() {
    // Obtain command line parameters
    let args = Args::parse();

    // Initialize logger
    let log_level = if args.verbose {
        #[cfg(debug_assertions)]
        {
            LevelFilter::Debug
        }
        #[cfg(not(debug_assertions))]
        {
            LevelFilter::Info
        }
    } else {
        LevelFilter::Warn
    };
    Logger::try_with_env_or_str(log_level.to_string())
        .unwrap()
        .log_to_stderr()
        .set_palette(String::from("196;226;44;91;-"))
        .start()
        .unwrap();

    let mut interpreter = Interpreter::new_from_path(
        &args.filename,
        InterpreterOptions::release().with_cell_size(args.cell_size),
    )
    .unwrap_or_else(|_| exit(1));
    interpreter.set_stdout_echo(args.echo);

    log::info!("Start executing program...");
    interpreter.run_to_end();
    log::info!("Reached end of code data. Terminating...")
}
