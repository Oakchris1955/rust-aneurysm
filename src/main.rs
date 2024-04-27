use clap::{arg, command, value_parser, ArgMatches};
use std::{
    fs,
    io::{self, stdout, Write},
    ops::{Add, AddAssign, Deref, Sub, SubAssign},
    process::exit,
};

/// The default filename to use in case one isn't specified by the user
const DEFAULT_FILENAME: &str = "main.bf";

/// The default cell size to use in case one isn't specified by the user
const DEFAULT_CELL_SIZE: &str = "30000";

/// A recursive function that searches for loops within a BF file
fn get_loop(code: &Vec<char>, begin: usize, loops: &mut Vec<(usize, usize)>) -> usize {
    // Begin reading the code from a parameter index
    let mut index = begin;

    // Loop through each char in the Vec, beginning from the parameter index
    while index < code.len() {
        // Obtain the corresponding character
        let character = code
            .get(index)
            .expect("Unexpected error while trying to index loops. Please report this error");

        match character {
            // If it is the beginning of the loop, run the same function, BUT begin on a different index.
            // Also, when the function is done executing, set the index to a later one to skip the already-processed loops
            '[' => index = get_loop(code, index + 1, loops),
            // If it is the end of the loop, push a new loop tuple into the loops Vec and return with the current index
            ']' => {
                loops.push((begin - 1, index));
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

/// Simulates a number overflow or underflow (used for the data pointer)
#[derive(Clone, Copy)]
pub struct WrappingUInt {
    pub limit: usize,
    uint: usize,
}

impl WrappingUInt {
    fn new(uint: usize, limit: usize) -> Self {
        Self { limit, uint }
    }
}

impl Deref for WrappingUInt {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.uint
    }
}

impl Add<usize> for WrappingUInt {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            limit: self.limit,
            uint: ((self.uint + rhs) % self.limit),
        }
    }
}

impl AddAssign<usize> for WrappingUInt {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs
    }
}

impl Sub<usize> for WrappingUInt {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            limit: self.limit,
            uint: if self.uint >= rhs {
                self.uint - rhs
            } else {
                self.limit - 1 - ((rhs - self.uint - 1) % self.limit)
            },
        }
    }
}

impl SubAssign<usize> for WrappingUInt {
    fn sub_assign(&mut self, rhs: usize) {
        *self = *self - rhs
    }
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

#[cfg(test)]
pub mod tests {
    use crate::{get_loop, WrappingUInt};

    /// A test function that ensures that the [`get_loop`] function works correctly
    #[test]
    fn test_loop_finder() {
        // The cases against which to check
        const TEST_CASES: &[(&str, &[(usize, usize)])] = &[
            ("[]", &[(0, 1)]),
            (" []", &[(1, 2)]),
            ("[[] []]", &[(0, 6), (1, 2), (4, 5)]),
        ];

        // Initialize some variables
        let mut loops: Vec<(usize, usize)> = Vec::new();
        let mut failed_cases: Vec<&str> = Vec::new();

        // Run the tests. In case a test fails, DON'T PANIC, just push the failed case into the failed_cases Vec
        for (text, test_case) in TEST_CASES {
            loops.clear();

            get_loop(&text.chars().collect::<Vec<char>>(), 0, &mut loops);
            loops.sort();

            if test_case != &loops.as_slice() {
                failed_cases.push(&text)
            }
        }

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

    #[test]
    fn wrapping_add() {
        let wrapping = WrappingUInt::new(456, 1000);

        assert_eq!(*(wrapping + 544), 0);
        assert_eq!(*(wrapping + 543), 999);
        assert_eq!(*(wrapping + 657), 113);

        assert_eq!(*(wrapping + 1544), 0);
        assert_eq!(*(wrapping + 1543), 999);
        assert_eq!(*(wrapping + 1657), 113);
    }

    #[test]
    fn wrapping_sub() {
        let wrapping = WrappingUInt::new(456, 1000);

        assert_eq!(*(wrapping - 456), 0);
        assert_eq!(*(wrapping - 457), 999);
        assert_eq!(*(wrapping - 584), 872);

        assert_eq!(*(wrapping - 1456), 0);
        assert_eq!(*(wrapping - 1457), 999);
        assert_eq!(*(wrapping - 1584), 872);
    }
}

fn main() {
    // Obtain command line parameters
    let args: ArgMatches = command!()
        .about("A Brainf**k interpreter written in Rust with minimal dependencies")
        .arg(
            arg!([filename] "Brainf**k file to execute")
                .default_value(DEFAULT_FILENAME)
                .required(false),
        )
        .arg(
            arg!(-m --mem <memory> "The memory size in bytes/cells to allocate for the program")
                .default_value(DEFAULT_CELL_SIZE)
                .value_parser(value_parser!(usize))
                .required(false),
        )
        .arg(
            arg!(-v --verbose "Enable verbose logging")
                .value_parser(value_parser!(bool))
                .required(false),
        )
        .get_matches();

    // Obtain the filename from them
    let filename = args.get_one::<String>("filename").expect("Error trying to obtain name of file to execute. This error shouldn't happen by default, since a default value is specified. Please report this error");

    // Check whether or not verbose logging is enabled
    let verbose = args
        .get_one::<bool>("verbose")
        .expect("Error trying to see wheteher verbose logging is enabled or not")
        .clone();

    // Also, obtain the cell size
    let cell_size = *args
        .get_one::<usize>("mem")
        .expect("Error while parsing command line argument \"memory\" to an integer");

    // Read file contents (or terminate if an error occurs while doing so)
    let mut code = fs::read_to_string(filename)
        .unwrap_or_else(|error| {
            match error.kind() {
                io::ErrorKind::NotFound => eprintln!("File {} not found", filename),
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Couldn't open file due to a permission error")
                }
                _ => eprintln!("An unknown error occured while opening file {}", filename),
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

    if verbose {
        println!("Successfully opened file {}", filename);
    }

    // Initialize a Vec to store the loops' start and end
    let mut loops: Vec<(usize, usize)> = Vec::new();

    // Obtain loops' data
    get_loop(&code, 0, &mut loops);

    // Allocate some memory for the data array, as well as the data pointer and the instruction pointer
    let mut data_pointer: WrappingUInt = WrappingUInt::new(0, cell_size);
    let mut instruction_pointer: usize = 0;

    let mut data: Vec<u8> = vec![0; cell_size];
    // Creating a new vector might not allocate any memory
    // For this reason, we iterate through the vector and set all its items to 0
    if verbose {
        print!("Allocating memory... ");
        // Flush to stdout
        let _ = stdout().flush();
    }
    data.iter_mut().for_each(|cell| *cell = 0);
    if verbose {
        println!("done");

        println!("Start executing program...")
    }

    // Loop through each character and process it accordingly
    while instruction_pointer < code.len() {
        let character = code
            .get(instruction_pointer)
            .expect("Program reached EOF before it was expected");

        match character {
            '>' => data_pointer += 1,
            '<' => data_pointer -= 1,
            '+' => data[*data_pointer] = data[*data_pointer].overflowing_add(1).0,
            '-' => data[*data_pointer] = data[*data_pointer].overflowing_sub(1).0,
            '.' => print!("{}", data[*data_pointer] as char),
            ',' => data[*data_pointer] = read_char() as u8,
            '[' => {
                if data[*data_pointer] == 0 {
                    instruction_pointer = loops
                        .iter()
                        .find(|(first, _)| first == &instruction_pointer)
                        .unwrap()
                        .1
                }
            }
            ']' => {
                if data[*data_pointer] != 0 {
                    instruction_pointer = loops
                        .iter()
                        .find(|(_, second)| second == &instruction_pointer)
                        .unwrap()
                        .0
                }
            }
            _ => (),
        };

        instruction_pointer += 1;
    }

    if verbose {
        println!("Reached end of code data. Terminating...")
    }
}
