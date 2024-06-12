use std::io;

/// Read a single [`char`] from the [`Stdin`](`io::Stdin`)
pub fn read_char() -> char {
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
