use std::io;

use bimap::BiMap;
use log::info;
// yep, we need an external crate to format numbers with separators
use thousands::Separable;

use crate::{input::*, modular::Modular};

/// The default filename to use in case one isn't specified by the user
pub const DEFAULT_FILENAME: &str = "main.bf";

/// The default cell size to use in case one isn't specified by the user
pub const DEFAULT_CELL_SIZE: usize = 30000;

type Loops = BiMap<usize, usize>;

pub struct Interpreter<'a, 'b> {
    pub instruction_pointer: usize,
    pub data_pointer: Modular<usize>,

    pub code: Vec<char>,
    pub loops: Loops,
    pub data: Vec<u8>,

    /// If this is unset, will write to stdout
    pub sink: Option<&'a mut dyn io::Write>,
    /// If this is unset, will read from stdin
    pub source: Option<&'b mut dyn io::Read>,
}

impl<'a, 'b> Interpreter<'a, 'b> {
    pub fn new(code: String, num_of_cells: usize) -> Self {
        // turn the code String into a char vector
        let mut code = code.chars().collect::<Vec<char>>();

        // Remove all non-instruction characters
        Self::remove_comments(&mut code);

        // create a loop bimap and populate it
        let mut loops = Loops::new();
        Self::get_loop(&code, 0, &mut loops);

        info!("Allocating memory... ");
        // Creating a new data vector might not allocate any memory
        // For this reason, we iterate through the vector and set all its items to 0
        let mut data: Vec<u8> = vec![0_u8; num_of_cells];
        data.iter_mut().for_each(|cell| *cell = 0);
        info!(
            "Allocated {} bytes in total",
            // In the 22nd General Conference on Weights and Measures, it was declared that:
            // numbers may be divided in groups of three in order to facilitate reading;
            // neither dots nor commas are ever inserted in the spaces between groups
            num_of_cells.separate_with_spaces()
        );

        Self {
            instruction_pointer: 0,
            data_pointer: Modular::with_limit(num_of_cells),

            code,
            loops,
            data,

            source: None,
            sink: None,
        }
    }

    /// If this returns `None`, EOF was reached
    pub fn run_cycle(&mut self) -> Option<()> {
        // Check if EOF was reached
        if self.instruction_pointer >= self.code.len() {
            return None;
        }

        // Get the next character to process
        let character = self.code[self.instruction_pointer];

        // Loop through each character and process it accordingly
        match character {
            '>' => self.data_pointer += 1,
            '<' => self.data_pointer -= 1,
            '+' => {
                self.data[*self.data_pointer] = self.data[*self.data_pointer].overflowing_add(1).0
            }
            '-' => {
                self.data[*self.data_pointer] = self.data[*self.data_pointer].overflowing_sub(1).0
            }
            '.' => match &mut self.sink {
                Some(writable) => writable
                    .write_all(&[self.data[*self.data_pointer]])
                    .unwrap(),
                None => print!("{}", self.data[*self.data_pointer] as char),
            },
            ',' => match &mut self.source {
                Some(readable) => {
                    let mut buf = [0u8];
                    readable.read_exact(&mut buf).unwrap();
                    self.data[*self.data_pointer] = buf[0];
                }
                None => self.data[*self.data_pointer] = read_char() as u8,
            },
            '[' => {
                if self.data[*self.data_pointer] == 0 {
                    self.instruction_pointer =
                        *self.loops.get_by_left(&self.instruction_pointer).unwrap()
                }
            }
            ']' => {
                if self.data[*self.data_pointer] != 0 {
                    self.instruction_pointer =
                        *self.loops.get_by_right(&self.instruction_pointer).unwrap()
                }
            }
            _ => (),
        };

        // Increment the instruction pointer for the next cycle
        self.instruction_pointer += 1;

        Some(())
    }

    /// Runs `run_cycle` until it returns `None`
    pub fn run_to_end(&mut self) {
        while self.run_cycle().is_some() {}
    }

    /// An easy way to redirect the program's character output
    #[allow(dead_code)]
    pub fn set_sink<W>(&mut self, sink: &'a mut W)
    where
        W: io::Write,
    {
        self.sink = Some(sink)
    }

    /// An easy way to set an alternative program character input
    #[allow(dead_code)]
    pub fn set_source<R>(&mut self, source: &'b mut R)
    where
        R: io::Read,
    {
        self.source = Some(source)
    }

    /// Remove all non-instruction characters
    fn remove_comments(code: &mut Vec<char>) {
        code.retain(|c| match c {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
            _ => false,
        })
    }

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
                '[' => index = Self::get_loop(code, index + 1, loops),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// A test function that ensures that the [`get_loop`] function works correctly
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

            Interpreter::get_loop(&text.chars().collect::<Vec<char>>(), 0, &mut loops);

            // Convert BiMap to a vector
            let mut loop_slice: Vec<(usize, usize)> =
                loops.iter().map(|(left, right)| (*left, *right)).collect();

            // Sort that vector
            loop_slice.sort();

            if test_case != &loop_slice {
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
}
