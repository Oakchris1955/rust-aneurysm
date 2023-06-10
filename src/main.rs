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
}
