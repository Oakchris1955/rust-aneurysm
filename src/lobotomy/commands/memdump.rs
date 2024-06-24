use std::error::Error;

use clap::CommandFactory;
use clap::Parser;
use displaydoc::Display;
use thiserror;

use term_size::dimensions as term_dimensions;

use crate::StateType;

const ABOUT: &str = "Dumps a portion of the programs memory";
const LONG_ABOUT: &str = concat!(
    "Also \x1B[7mhighlights\x1B[0m at which cell the data pointer is, if that cell happens to be in range",
    "\n\n",
    "The memory will be displayed in a table-like format, the first row showing the last two digits of each index for each column ",
    "and the last one the corresponding cell's content in hex"
);

#[derive(Parser, Debug)]
#[command(
    bin_name = "memdump",
    about = ABOUT,
    long_about = format!("{}\n{}", ABOUT, LONG_ABOUT)
)]
pub struct MemdumpArgs {
    /// The "width" of the memory dump. Must be an odd number
    #[arg(short, long, default_value_t = 9)]
    width: usize,

    /// Print the memory contents as uppercase hex
    #[arg(short = 'H', long)]
    uppercase_hex: bool,

    /// The start offset of the memdump
    #[arg(default_value_t = 0)]
    offset: usize,
}

// +1 for the non-existent end seperator and /3 for the cell char size plus the seperator
fn max_cells_visible(width: usize) -> usize {
    (width + 1) / 3
}

enum CellType {
    Index(usize),
    Data { byte: u8, hex_uppercase: bool },
}

fn print_cell(cell: CellType, inverse: bool, sep: bool) {
    // start inverse ANSI mode
    if inverse {
        print!("\x1B[7m")
    }

    match cell {
        CellType::Index(i) => {
            // show only the last 2 digits of the current offset for compatibility reasons
            print!("{:02}", i % 100)
        }
        CellType::Data {
            byte,
            hex_uppercase,
        } => {
            // print it as a 2-digit hex
            if !hex_uppercase {
                print!("{:02x}", byte)
            } else {
                print!("{:02X}", byte)
            }
        }
    }

    // reset all enabled ANSI modes
    if inverse {
        print!("\x1B[0m")
    }

    // print a separator between numbers
    if sep {
        print!("|")
    }
}

pub fn memdump(state: &mut StateType, args: MemdumpArgs) -> Result<(), Box<dyn Error>> {
    if args.width % 2 == 0 {
        let mut cmd = MemdumpArgs::command();
        cmd.error(
            clap::error::ErrorKind::ValueValidation,
            MemdumpError::EvenWidth,
        )
        .print()
        .unwrap();
        return Ok(());
    }

    let dimensions = term_dimensions().unwrap();
    if args.width > max_cells_visible(dimensions.0) {
        eprintln!(
            "{}",
            MemdumpError::TerminalTooSmall {
                width: dimensions.0,
                cells: max_cells_visible(dimensions.0),
                provided: args.width
            }
        );
        return Ok(());
    }

    // we won't be mutating anything, so this is a normal borrow
    let state = state.borrow();

    // Let's now check if the offset parameter, combined with width, is in bounds of the cell array
    if args.offset + args.width > state.interpreter.data.len() {
        eprintln!(
            "{}",
            MemdumpError::OutOfBounds {
                limit: args.offset + args.width,
                overflown: args.offset + args.width - state.interpreter.data.len(),
                end: state.interpreter.data.len(),
            }
        );
        return Ok(());
    }

    // we can now start dumping the memory
    let start: usize = args.offset;
    let end: usize = args.offset + args.width;

    // this could probably look better, but it works and is readable. if u have found a cleaner way, open a PR
    for i in start..=end {
        print_cell(
            CellType::Index(i),
            i == *state.interpreter.data_pointer,
            i != end,
        )
    }
    println!();

    for i in start..=end {
        print_cell(
            CellType::Data {
                byte: state.interpreter.data[i],
                hex_uppercase: args.uppercase_hex,
            },
            i == *state.interpreter.data_pointer,
            i != end,
        )
    }
    println!();

    Ok(())
}

#[derive(Display, thiserror::Error, Debug)]
pub enum MemdumpError {
    /// Expected the width parameter to be odd, it is even
    EvenWidth,
    /** Resize the terminal so that the program can print the entire memdump
     * (With the current terminal width of {width}, a max number of {cells} cells can be printed, found {provided})
     */
    TerminalTooSmall {
        width: usize,
        cells: usize,
        provided: usize,
    },
    /// Memdump end limit is out-of-bounds: The end limit ({limit}) is {overflown} bytes after the cell array's end ({end})
    OutOfBounds {
        limit: usize,
        overflown: usize,
        end: usize,
    },
}
