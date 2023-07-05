use std::cmp::{max, min};

pub fn print_chars(from: char, to: char) {
    let min_char = min(from, to);
    let max_char = max(from, to);
    for item in min_char..=max_char {
        print!("{item}");
    }
}
