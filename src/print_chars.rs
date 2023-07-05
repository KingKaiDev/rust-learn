use std::cmp::{max, min};

pub fn print_chars(form: char, to: char) {
    let min_char = min(form, to);
    let max_char = max(form, to);
    for item in min_char..=max_char {
        print!("{item}");
    }
}
