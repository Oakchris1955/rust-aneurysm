use super::*;

use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use bimap::BiMap;
use displaydoc::Display;
use log;

use num_modular::Reducer;

type Loops = BiMap<usize, usize>;

pub struct Interpreter<'a, 'b> {
    pub instruction_pointer: usize,
    pub data_pointer: usize,
    data_modulo: num_modular::Vanilla<usize>,

    pub code: Vec<char>,
    pub loops: Loops,
    pub data: Vec<u8>,

    profile: InterpreterProfile,

    sink: Sink<'a>,
    source: Source<'b>,

    _stdout_echo: bool,
}

enum Sink<'a> {
    Stdout(io::Stdout),
    Other(&'a mut dyn io::Write),
}

impl<'a> Default for Sink<'a> {
    fn default() -> Self {
        Self::Stdout(io::stdout())
    }
}

impl<'a> io::Write for Sink<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Sink::Stdout(stdout) => return stdout.write(buf),
            Sink::Other(other) => return other.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Sink::Stdout(stdout) => return stdout.flush(),
            Sink::Other(other) => return other.flush(),
        }
    }
}

enum Source<'b> {
    Stdin(io::Stdin),
    Other(&'b mut dyn io::Read),
}

impl<'a> Default for Source<'a> {
    fn default() -> Self {
        Self::Stdin(io::stdin())
    }
}

impl<'b> io::Read for Source<'b> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Source::Stdin(stdin) => return stdin.read(buf),
            Source::Other(other) => return other.read(buf),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum InterpreterProfile {
    Debug,
    Release,
}

impl Default for InterpreterProfile {
    fn default() -> Self {
        #[cfg(not(debug_assertions))]
        {
            InterpreterProfile::Release
        }
        #[cfg(debug_assertions)]
        {
            InterpreterProfile::Debug
        }
    }
}

pub struct InterpreterOptions {
    num_of_cells: usize,
    profile: InterpreterProfile,
}

impl InterpreterOptions {
    pub fn debug() -> Self {
        Self {
            profile: InterpreterProfile::Debug,
            ..Self::default()
        }
    }

    pub fn release() -> Self {
        Self {
            profile: InterpreterProfile::Release,
            ..Self::default()
        }
    }

    pub fn with_cell_size(mut self, cell_size: usize) -> Self {
        self.num_of_cells = cell_size;
        self
    }
}

impl Default for InterpreterOptions {
    fn default() -> Self {
        Self {
            num_of_cells: DEFAULT_CELL_SIZE,
            profile: InterpreterProfile::default(),
        }
    }
}

impl<'a, 'b> Interpreter<'a, 'b> {
    pub fn new<S>(code: S, options: InterpreterOptions) -> InterpreterResult<Self>
    where
        S: ToString,
    {
        // turn the code String into a char vector
        let mut code = code.to_string().chars().collect::<Vec<char>>();

        // Remove all non-instruction characters
        if options.profile == InterpreterProfile::Release {
            Self::remove_comments(&mut code);
        }

        log::debug!("Allocating memory... ");
        // Creating a new data vector might not allocate any memory
        // For this reason, we iterate through the vector and set all its items to 0
        #[cfg(debug_assertions)]
        if options.num_of_cells >= 10_000_000 {
            log::warn!(
                "The program is allocating a significant amount of memory in debug mode ({} bytes). ",
                options.num_of_cells
            );
            log::warn!(
                "This allocation may take a long time, if it is well above 100 MBs, please run the program in release mode instead when performing such large allocations"
            );
            log::warn!(
                "Apart from the memory allocation itself, if you are running an exhaustive program, it might take a long time to finish"
            );
            log::warn!(
                "Generally, if your memory space is more than 10 MBs, please use release mode"
            )
        }

        let mut data: Vec<u8> = vec![0_u8; options.num_of_cells];
        data.iter_mut().for_each(|cell| *cell = 0);
        log::debug!(
            "Allocated {} bytes in total",
            // In the 22nd General Conference on Weights and Measures, it was declared that:
            // numbers may be divided in groups of three in order to facilitate reading;
            // neither dots nor commas are ever inserted in the spaces between groups
            options.num_of_cells
        );

        Ok(Self {
            instruction_pointer: 0,
            data_pointer: 0,
            data_modulo: num_modular::Vanilla::new(&options.num_of_cells),

            loops: Self::get_loop(&code)?,
            code,
            data,

            profile: options.profile,

            source: Source::default(),
            sink: Sink::default(),

            _stdout_echo: false,
        })
    }

    pub fn new_from_path<P>(path: P, options: InterpreterOptions) -> InterpreterResult<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        // Read file contents (or return an error if one occurs while doing so)
        match fs::read_to_string(path) {
            Ok(code) => {
                log::info!("Successfully opened file {}", path.display());
                Self::new(code, options)
            }
            Err(error) => {
                match error.kind() {
                    io::ErrorKind::NotFound => log::error!("File \"{}\" not found", path.display()),
                    io::ErrorKind::PermissionDenied => {
                        log::error!("Couldn't open file due to a permission error")
                    }
                    _ => log::error!(
                        "An unknown error occured while opening file {}",
                        path.display()
                    ),
                };
                Err(InterpreterError::IOError(error))
            }
        }
    }

    /// If this returns `None`, EOF was reached
    pub fn run_step(&mut self) -> Result<Option<()>, InterpreterError> {
        // Check if EOF was reached
        if self.instruction_pointer >= self.code.len() {
            return Ok(None);
        }

        // Get the next character to process
        let character = self.code[self.instruction_pointer];

        // Loop through each character and process it accordingly
        match character {
            '>' => self.data_modulo.add_in_place(&mut self.data_pointer, &1),
            '<' => self.data_modulo.sub_in_place(&mut self.data_pointer, &1),
            '+' => self.data[self.data_pointer] = self.data[self.data_pointer].overflowing_add(1).0,
            '-' => self.data[self.data_pointer] = self.data[self.data_pointer].overflowing_sub(1).0,
            '.' => self.sink.write_all(&[self.data[self.data_pointer]])?,
            ',' => {
                let mut buf = [0u8];
                let bytes_read = self.source.read(&mut buf)?;
                if bytes_read == 0 {
                    /*
                     * we have reached EOF on the source
                     * per the esolang wiki, we can either leave the current
                     * cell unchanged or return with a status code of zero, if
                     * we were a program. Since we are in a library, we just exit
                     * as if we reached the instructions' EOF
                     */
                    return Ok(None);
                }
                self.data[self.data_pointer] = buf[0];

                if self._stdout_echo {
                    let mut stdout = io::stdout();
                    stdout.write_all(&buf)?;
                    stdout.flush()?;
                }
            }
            '[' => {
                if self.data[self.data_pointer] == 0 {
                    self.instruction_pointer =
                        *self.loops.get_by_left(&self.instruction_pointer).unwrap()
                }
            }
            ']' => {
                if self.data[self.data_pointer] != 0 {
                    self.instruction_pointer =
                        *self.loops.get_by_right(&self.instruction_pointer).unwrap()
                }
            }
            _ => (),
        };

        // Increment the instruction pointer for the next cycle
        self.instruction_pointer += 1;

        Ok(Some(()))
    }

    /// Runs `run_step` until it returns `None`
    pub fn run_to_end(&mut self) -> Result<(), InterpreterError> {
        while self.run_step()?.is_some() {}

        Ok(())
    }

    /// Ready the interpreter for another program run
    pub fn reset(&mut self) {
        // Reset instruction and data pointer
        self.instruction_pointer = 0;
        self.data_pointer = 0;

        // Reset data vector
        self.data.iter_mut().for_each(|cell| *cell = 0);

        log::debug!("Program state successfully reset");
    }

    /// An easy way to redirect the program's character output
    #[allow(dead_code)]
    pub fn set_sink<W>(&mut self, sink: &'a mut W)
    where
        W: io::Write,
    {
        self.sink = Sink::Other(sink)
    }

    /// An easy way to set an alternative program character input
    #[allow(dead_code)]
    pub fn set_source<R>(&mut self, source: &'b mut R)
    where
        R: io::Read,
    {
        self.source = Source::Other(source)
    }

    // Whether to echo data written to stdin back to stdout IF AND ONLY IF sink isn't set
    pub fn set_stdout_echo(&mut self, echo: bool) {
        self._stdout_echo = echo
    }

    pub fn get_options(&self) -> InterpreterOptions {
        InterpreterOptions {
            num_of_cells: self.data_modulo.modulus(),
            profile: self.profile.clone(),
        }
    }

    /// Remove all non-instruction characters
    fn remove_comments(code: &mut Vec<char>) {
        code.retain(|c| match c {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
            _ => false,
        })
    }

    /// A looping function to get all matching loop brackets (returns [`InterpreterError::UnmatchedLoop`] if a bracket is unmatched)
    fn get_loop(code: &Vec<char>) -> Result<Loops, InterpreterError> {
        let mut loops = BiMap::new();

        let mut stack: Vec<usize> = Vec::new();

        for (index, char) in code.iter().enumerate() {
            match char {
                '[' => stack.push(index),
                ']' => {
                    loops.insert(stack.pop().ok_or(InterpreterError::UnmatchedLoop)?, index);
                }
                _ => (),
            }
        }

        if !stack.is_empty() {
            return Err(InterpreterError::UnmatchedLoop);
        }

        Ok(loops)
    }
}

pub type InterpreterResult<T> = Result<T, InterpreterError>;

#[derive(Display, Debug)]
pub enum InterpreterError {
    /// Found unmatched loop brackets
    UnmatchedLoop,
    /// {0}
    IOError(io::Error),
}

impl From<io::Error> for InterpreterError {
    fn from(value: io::Error) -> Self {
        InterpreterError::IOError(value)
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
        let mut failed_cases: Vec<&str> = Vec::new();

        // Run the tests. In case a test fails, DON'T PANIC, just push the failed case into the failed_cases Vec
        for (text, test_case) in TEST_CASES {
            let loops = Interpreter::get_loop(&text.chars().collect::<Vec<char>>()).unwrap();

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

    #[test]
    /// If the "Hello World!" program runs, then so does probably everything else
    /// Apart from testing if the interpreter actually works, it also checks if the sink is working
    fn hello_world() {
        // https://esolangs.org/wiki/Brainfuck#Hello,_World!
        const PROGRAM: &str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

        let mut output: Vec<u8> = Vec::new();
        let mut interpreter = Interpreter::new(PROGRAM, InterpreterOptions::release()).unwrap();
        interpreter.set_sink(&mut output);
        interpreter.run_to_end().unwrap();

        assert_eq!(output.as_slice(), b"Hello World!\n")
    }

    #[test]
    /// Test if the source and sink are functioning, and that stdout_echo do indeed work ONLY for stdout
    fn cat() {
        const INPUT: &str = "Hello, cat!";
        // for those wonder why we put that many + before the loop, that's the loop counter
        // it shows how many times the loop is gonna execute, or in our case, read from source
        let program = format!("{}[>,.<-]", "+".repeat(INPUT.len()));

        let mut output: Vec<u8> = Vec::new();
        let mut input = io::Cursor::new(INPUT);
        let mut interpreter = Interpreter::new(program, InterpreterOptions::release()).unwrap();
        interpreter.set_source(&mut input);
        interpreter.set_sink(&mut output);
        interpreter.set_stdout_echo(true);
        interpreter.run_to_end().unwrap();

        assert_eq!(output.as_slice(), INPUT.as_bytes())
    }
}
