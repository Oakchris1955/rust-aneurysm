use std::fs;

const FILENAME: &str = "main.bf";

fn get_loop(code: &String, begin: usize, loops: &mut Vec<(usize, usize)>) -> usize {
    let mut index = begin;

    while index < code.len() {
        let character = code
            .chars()
            .nth(index)
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

    code.len()
}

fn main() {
    let code =
        fs::read_to_string(FILENAME).expect(format!("Couldn't read file {}", FILENAME).as_str());

    println!("Successfully opened file {}", FILENAME);

    let mut loops: Vec<(usize, usize)> = Vec::new();

    get_loop(&code, 0, &mut loops);
}
