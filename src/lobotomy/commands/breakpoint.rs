use std::error::Error;

use clap::CommandFactory;
use clap::{Parser, ValueEnum};
use displaydoc::Display;
use log;
use thiserror;

use crate::StateType;

#[derive(ValueEnum, Debug, Clone, Default)]
pub enum BreakpointMode {
    /// Add a new breakpoint
    Add,
    /// Remove an already-existing breakpoint
    Remove,
    /// Show a list of all breakpoints
    #[default]
    Show,
}

#[derive(Parser, Debug)]
#[command(name = "breakpoint", about = "An easy way to tweak breakpoints")]
pub struct BreakpointArgs {
    /// What to do at the provided index
    #[arg(value_enum, default_value_t = BreakpointMode::default())]
    mode: BreakpointMode,
    /// The index of the breakpoint (required for add and remove modes)
    #[clap(num_args = 1.., value_delimiter = ' ')]
    index: Option<Vec<usize>>,
}

pub fn breakpoint(state: &mut StateType, args: BreakpointArgs) -> Result<(), Box<dyn Error>> {
    let mut state = state.borrow_mut();

    let indexes = match args.index {
        Some(index) => match args.mode {
            BreakpointMode::Show => {
                let mut cmd = BreakpointArgs::command();
                cmd.error(
                    clap::error::ErrorKind::ArgumentConflict,
                    "\x1B[1m'index'\x1B[0m argument cannot be used in \x1B[1mSHOW\x1B[0m mode",
                )
                .print()
                .unwrap();
                return Ok(());
            }
            _ => index,
        },
        None => match args.mode {
            BreakpointMode::Add | BreakpointMode::Remove => {
                let mut cmd = BreakpointArgs::command();
                cmd.error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "In the current mode, the \x1B[1mINDEX\x1B[0m argument is required",
                )
                .print()
                .unwrap();
                return Ok(());
            }
            // We won't be using this, so we don't care
            _ => Default::default(),
        },
    };

    match args.mode {
        BreakpointMode::Add => {
            for index in indexes {
                // don't insert anything past EOF
                if index >= state.interpreter.code.len() {
                    eprintln!(
                        "{}",
                        BreakpointError::PastEOF {
                            index,
                            len: state.interpreter.code.len()
                        }
                    );
                }

                if let Err(insert_index) = state.breakpoints.binary_search(&index) {
                    state.breakpoints.insert(insert_index, index);
                    println!("Inserted breakpoint at index {}", index);
                    log::debug!("Inserted breakpoint at index {}", index);
                } else {
                    eprintln!("{}", BreakpointError::AlreadyExists(index));
                }
            }
        }
        BreakpointMode::Remove => {
            for index in indexes {
                if let Ok(insert_index) = state.breakpoints.binary_search(&index) {
                    state.breakpoints.remove(insert_index);
                    println!("Removed breakpoint at index {}", index);
                    log::debug!("Removed breakpoint at index {}", index);
                } else {
                    eprintln!("{}", BreakpointError::NotFound(index));
                }
            }
        }
        BreakpointMode::Show => {
            println!("{:?}", state.breakpoints);
        }
    }
    Ok(())
}

#[derive(Display, thiserror::Error, Debug)]
pub enum BreakpointError {
    /// There already exists a breakpoint at the index {0}
    AlreadyExists(usize),
    /// This index ({index}) is past/at the program's EOF (located at {len}). Not inserting breakpoint
    PastEOF { index: usize, len: usize },
    /// There isn't a breakpoint at the index {0}
    NotFound(usize),
}
