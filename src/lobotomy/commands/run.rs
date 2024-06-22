use std::error::Error;

use clap::Parser;
use displaydoc::Display;
use log;
use thiserror;

use crate::StateType;

#[derive(Parser, Debug)]
#[command(name = "run", about = "Start executing the program")]
pub struct RunArgs {
    #[arg(short, long, exclusive(true))]
    /// Whether to ignore breakpoints (basically runs the program till EOF)
    ignore_breakpoints: bool,

    #[arg(short, long)]
    /// Reset the interpreter to its original state (allows you to run the program again)
    reset: bool,
}

pub fn run(state: &mut StateType, args: RunArgs) -> Result<(), Box<dyn Error>> {
    let mut state = state.borrow_mut();

    if args.reset {
        state.interpreter.reset()
    }

    if state.interpreter.instruction_pointer >= state.interpreter.code.len() {
        eprintln!("{}", RunError::PastEOF);
        return Ok(());
    }

    if args.ignore_breakpoints || state.breakpoints.is_empty() {
        state.interpreter.run_to_end();
        eprintln!("\n{}", RunError::ReachedEOF);
        return Ok(());
    } else {
        // Check where the next breakpoint would be
        let next_breakpoint_index = *state
            .breakpoints
            .get(
                // find where the next breakpoint is located inside state.breakpoints
                state
                    .breakpoints
                    .binary_search(&state.interpreter.instruction_pointer)
                    .map(|ok_index| {
                        log::debug!(
                            "Found a breakpoint at the current instruction pointer index {}. This will be ignored and the next breakpoint (if any) will be expected",
                            state.interpreter.instruction_pointer
                        );
                        ok_index + 1
                    }) // get the next breakpoint, we already stopped here
                    .unwrap_or_else(|err_index| err_index),
            )
            .unwrap_or_else(|| {assert_ne!(state.interpreter.code.len(), usize::MAX, "Woah, just woah"); &usize::MAX}); // in this case, this is a "virtual" breakpoint that will never be reached, since it is past the program's EOF

        loop {
            if state.interpreter.run_cycle().is_none() {
                eprintln!("\n{}", RunError::ReachedEOF);

                return Ok(());
            }
            if state.interpreter.instruction_pointer >= next_breakpoint_index {
                eprintln!("\n{}", RunError::BreakpointFound(next_breakpoint_index),);

                return Ok(());
            }
        }
    }
}

#[derive(Display, thiserror::Error, Debug)]
pub enum RunError {
    // Not actually an error
    /// Found a breakpoint at index {0}
    BreakpointFound(usize),

    // Not actually an error
    /// Reached program EOF without finding any breakpoints
    ReachedEOF,

    /// The instruction pointer is past the program's EOF. Use the -r flag to reset it
    PastEOF,
}
