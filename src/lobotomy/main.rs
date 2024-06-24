use std::rc::Rc;
use std::{cell::RefCell, process::exit};

use clap::Parser;
use flexi_logger::{LogSpecification, Logger};
use log::LevelFilter;
use rustyline::DefaultEditor;
use shellfish::{handler::DefaultHandler, *};

use aneurysm::interpreter::*;

mod clap_parser;
mod commands;
mod dirs;
mod state;

use commands::*;
use dirs::*;
pub use state::*;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path to the file to debug
    filename: std::path::PathBuf,
}

pub type StateType<'a, 'b> = Rc<RefCell<State<'a, 'b>>>;

fn main() {
    let args = Args::parse();

    // before doing ANYTHING, configure the logger
    Logger::try_with_env_or_str(LogSpecification::info().to_string())
        .unwrap()
        .log_to_file(log_filespec())
        .duplicate_to_stderr(LevelFilter::Warn.into())
        .set_palette(String::from("196;226;44;91;-"))
        .start()
        .unwrap();

    let interpreter = match Interpreter::new_from_path(&args.filename, DEFAULT_CELL_SIZE) {
        Ok(interpreter) => interpreter,
        Err(err) => {
            log::error!(
                "An error occured while opening file \"{}\":\n{}",
                args.filename.display(),
                err
            );
            exit(1)
        }
    };
    log::debug!(
        "Successfully created interpreter from file \"{}\"",
        args.filename.display()
    );

    let state: StateType = Rc::new(RefCell::new(State::new(interpreter)));
    let prompt = Prompt::new(state.clone());

    let mut shell = Shell::new_with_handler(
        state.clone(),
        prompt,
        DefaultHandler::default(),
        DefaultEditor::new().unwrap(),
    );
    log::debug!("Shell object created, injecting commands...");

    shell.commands.insert(
        "breakpoint",
        clap_command!(StateType, BreakpointArgs, breakpoint),
    );

    shell
        .commands
        .insert("run", clap_command!(StateType, RunArgs, run));

    shell
        .commands
        .insert("memdump", clap_command!(StateType, MemdumpArgs, memdump));

    log::debug!("Commands injected, starting main loop...");

    shell.run().unwrap();

    log::debug!("Main loop interrupted by user. Terminating...");
}
