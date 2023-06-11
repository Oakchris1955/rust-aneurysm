use clap::{arg, command, ArgMatches};
use std::{fs, io, process::exit};

const DEFAULT_FILENAME: &str = "main.bf";

fn get_loop(code: &String, begin: usize, loops: &mut Vec<(usize, usize)>) -> usize {
    let mut index = begin;

    let chars = code.chars().collect::<Vec<char>>();

    while index < chars.len() {
        let character = chars
            .get(index)
            .expect("Unexpected error while trying to index loops. Please report this error");

        match character {
            '[' => index = get_loop(code, index + 1, loops),
            ']' => {
                loops.push((begin - 1, index));
                return index;
            }
            _ => (),
        };

        index += 1;
    }

    chars.len()
}

fn wrapping_change(first: usize, add: bool, limit: usize) -> usize {
    if first == limit - 1 && add {
        0
    } else if first == 0 && !add {
        limit - 1
    } else {
        if add {
            first + 1
        } else {
            first - 1
        }
    }
}

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
mod tests {
    use crate::get_loop;

    #[test]
    fn test_loop_finder() {
        const TEST_CASES: &[(&str, &[(usize, usize)])] = &[
            ("[]", &[(0, 1)]),
            (" []", &[(1, 2)]),
            ("[[] []]", &[(0, 6), (1, 2), (4, 5)]),
        ];

        let mut loops: Vec<(usize, usize)> = Vec::new();
        let mut failed_cases: Vec<&str> = Vec::new();

        for (text, test_case) in TEST_CASES {
            loops.clear();

            get_loop(&text.to_string(), 0, &mut loops);
            loops.sort();

            if test_case != &loops.as_slice() {
                failed_cases.push(&text)
            }
        }

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
    let args: ArgMatches = command!()
        .about("A Brainf**k interpreter written in Rust with minimal dependencies")
        .arg(
            arg!([filename] "Brainf**k file to execute")
                .default_value(DEFAULT_FILENAME)
                .required(false),
        )
        .get_matches();

    let filename = args.get_one::<String>("filename").expect("Error trying to obtain name of file to execute. This error shouldn't happen by default, since a default value is specified. Please report this error");

    let code = fs::read_to_string(filename).unwrap_or_else(|error| {
        match error.kind() {
            io::ErrorKind::NotFound => eprintln!("File {} not found", filename),
            io::ErrorKind::PermissionDenied => {
                eprintln!("Couldn't open file due to a permission error")
            }
            _ => eprintln!("An unknown error occured while opening file {}", filename),
        };

        exit(1)
    });

    println!("Successfully opened file {}", filename);

    let mut loops: Vec<(usize, usize)> = Vec::new();

    get_loop(&code, 0, &mut loops);

    // Allocate some memory for the data array, as well as the data pointer and the instruction pointer
    let mut data_pointer: usize = 0;
    let mut instruction_pointer: usize = 0;

    let mut data = [0 as u8; 30000];

    let chars = code.chars().collect::<Vec<char>>();

    while instruction_pointer < chars.len() {
        let character = chars
            .get(instruction_pointer)
            .expect("Program reached EOF before it was expected");

        match character {
            '>' => data_pointer = wrapping_change(data_pointer, true, 30000),
            '<' => data_pointer = wrapping_change(data_pointer, false, 30000),
            '+' => data[data_pointer] = data[data_pointer].overflowing_add(1).0,
            '-' => data[data_pointer] = data[data_pointer].overflowing_sub(1).0,
            '.' => print!("{}", data[data_pointer] as char),
            ',' => data[data_pointer] = read_char() as u8,
            '[' => {
                if data[data_pointer] == 0 {
                    instruction_pointer = loops
                        .iter()
                        .find(|(first, _)| first == &instruction_pointer)
                        .unwrap()
                        .1
                }
            }
            ']' => {
                if data[data_pointer] != 0 {
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
}
