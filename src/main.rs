use std::fs;

const FILENAME: &str = "main.bf";

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
    let code =
        fs::read_to_string(FILENAME).expect(format!("Couldn't read file {}", FILENAME).as_str());

    println!("Successfully opened file {}", FILENAME);

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
