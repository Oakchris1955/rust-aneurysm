use bimap::BiMap;
use clap::{arg, command, Parser};
// yep, we need an external crate to format numbers with separators
use thousands::Separable;

use log::{error, info, LevelFilter};
use simple_logger::SimpleLogger;

use std::{fs, io, process::exit};

mod modular;
use modular::*;

/// The default filename to use in case one isn't specified by the user
const DEFAULT_FILENAME: &str = "main.bf";

/// The default cell size to use in case one isn't specified by the user
const DEFAULT_CELL_SIZE: usize = 30000;

type Loops = BiMap<usize, usize>;

/// A recursive function that searches for loops within a BF file
fn get_loop(code: &Vec<char>, begin: usize, loops: &mut Loops) -> usize {
    // Begin reading the code from a parameter index
    let mut index = begin;

    // Loop through each char in the Vec, beginning from the parameter index
    while index < code.len() {
        // Obtain the corresponding character
        let character = code[index];

        match character {
            // If it is the beginning of the loop, run the same function, BUT begin on a different index.
            // Also, when the function is done executing, set the index to a later one to skip the already-processed loops
            '[' => index = get_loop(code, index + 1, loops),
            // If it is the end of the loop, push a new loop tuple into the loops Vec and return with the current index
            ']' => {
                loops.insert(begin - 1, index);
                return index;
            }
            _ => (),
        };

        // Increment index by one
        index += 1;
    }

    // If no loop ending found, assume that the loop ending is at EOF
    code.len()
}

/// Read a single [`char`] from the [`Stdin`](`io::Stdin`)
fn read_char() -> char {
    let mut input = String::new();

    loop {
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        if let Some(character) = input.chars().next() {
            return character;
        }
    }
}

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

#[cfg(test)]
pub mod tests {
    use crate::{get_loop, Loops};
    use bimap::BiMap;

    /// A test function that ensures that the [`get_loop`] function works correctly
    #[test]
    fn test_loop_finder() {
        // The cases against which to check
        static TEST_CASES: &[(&str, &[(usize, usize)])] = &[
            ("[]", &[(0, 1)]),
            (" []", &[(1, 2)]),
            ("[[] []]", &[(0, 6), (1, 2), (4, 5)]),
        ];

        // Initialize some variables
        let mut loops: Loops = BiMap::new();
        let mut failed_cases: Vec<&str> = Vec::new();

        // Run the tests. In case a test fails, DON'T PANIC, just push the failed case into the failed_cases Vec
        for (text, test_case) in TEST_CASES {
            loops.clear();

            get_loop(&text.chars().collect::<Vec<char>>(), 0, &mut loops);

            // Convert BiMap to a vector
            let mut loop_slice: Vec<(usize, usize)> =
                loops.iter().map(|(left, right)| (*left, *right)).collect();

            // Sort that vector
            loop_slice.sort();

            if test_case != &loop_slice {
                failed_cases.push(&text)
            }
        }

        get_loop(
            &TEST_CASES[1].0.chars().collect::<Vec<char>>(),
            0,
            &mut loops,
        );

        // If there is at least 1 fail, panic with a custom message
        if !failed_cases.is_empty() {
            panic!(
                "A total of {} out of {} tests failed.\n\nThese are the cases that failed:\n{}",
                failed_cases.len(),
                TEST_CASES.len(),
                failed_cases
                    .iter()
                    .map(|text| format!("{}\n", text))
                    .collect::<String>()
            )
        }
    }
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
    let mut code = fs::read_to_string(&args.filename)
        .unwrap_or_else(|error| {
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
        })
        .chars()
        .collect::<Vec<char>>();

    // Remove all non-instruction characters
    code.retain(|c| match c {
        '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
        _ => false,
    });

    info!("Successfully opened file {}", &args.filename);

    // Initialize a Vec to store the loops' start and end
    let mut loops: Loops = BiMap::new();

    // Obtain loops' data
    get_loop(&code, 0, &mut loops);

    // Allocate some memory for the data array, as well as the data pointer and the instruction pointer
    let mut data_pointer: Modular<_> = Modular::new(0, args.cell_size);
    let mut instruction_pointer: usize = 0;

    info!("Allocating memory... ");
    let mut data: Vec<u8> = vec![0; args.cell_size];
    // Creating a new vector might not allocate any memory
    // For this reason, we iterate through the vector and set all its items to 0
    data.iter_mut().for_each(|cell| *cell = 0);

    info!(
        "Allocated {} bytes in total",
        // In the 22nd General Conference on Weights and Measures, it was declared that:
        // numbers may be divided in groups of three in order to facilitate reading;
        // neither dots nor commas are ever inserted in the spaces between groups
        args.cell_size.separate_with_spaces()
    );

    info!("Start executing program...");

    // Loop through each character and process it accordingly
    while instruction_pointer < code.len() {
        let character = code[instruction_pointer];

        match character {
            '>' => data_pointer += 1,
            '<' => data_pointer -= 1,
            '+' => data[*data_pointer] = data[*data_pointer].overflowing_add(1).0,
            '-' => data[*data_pointer] = data[*data_pointer].overflowing_sub(1).0,
            '.' => print!("{}", data[*data_pointer] as char),
            ',' => data[*data_pointer] = read_char() as u8,
            '[' => {
                if data[*data_pointer] == 0 {
                    instruction_pointer = *loops.get_by_left(&instruction_pointer).unwrap()
                }
            }
            ']' => {
                if data[*data_pointer] != 0 {
                    instruction_pointer = *loops.get_by_right(&instruction_pointer).unwrap()
                }
            }
            _ => (),
        };

        instruction_pointer += 1;
    }

    info!("Reached end of code data. Terminating...")
}
