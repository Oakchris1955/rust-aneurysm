// Reset the interpreter to its original state (allows you to run the program again)

use std::error::Error;

use clap::{Parser, ValueEnum};
use displaydoc::Display;
use thiserror;

use crate::StateType;
use aneurysm::interpreter::*;

#[derive(ValueEnum, Debug, Clone)]
pub enum ReloadMode {
    /// Reload the file's contents
    File,
    /// Reset the interpreter to its original state (previously known as "run -r")
    Interpreter,
}

#[derive(Parser, Debug)]
#[command(
    bin_name = "reload",
    about = "Reload the file's contents or reset the interpreter"
)]
pub struct ReloadArgs {
    /// What mode to run the command in
    #[arg(value_enum)]
    mode: ReloadMode,
}

pub fn reload(state: &mut StateType, args: ReloadArgs) -> Result<(), Box<dyn Error>> {
    let mut state = state.borrow_mut();

    match args.mode {
        ReloadMode::Interpreter => {
            state.interpreter.reset();
            println!("Successfully reset interpreter")
        }
        ReloadMode::File => {
            state.interpreter = match Interpreter::new_from_path(
                state.filepath.clone(),
                state.interpreter.get_options(),
            ) {
                Ok(new_interpreter) => {
                    println!("File \"{}\" reloaded", state.filename());
                    // also don't forget to clean our breakpoints
                    state.breakpoints.clear();
                    new_interpreter
                }
                Err(interpreter_err) => {
                    eprintln!(
                        "An error occured while trying to reload the interpreter object for file {}. Keeping program state as-is.\nError message:\n{}",
                        state.filename(),
                        ReloadErr::InterpreterError(interpreter_err)
                    );
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

#[derive(Display, thiserror::Error, Debug)]
pub enum ReloadErr {
    /// {0}
    InterpreterError(InterpreterError),
}
